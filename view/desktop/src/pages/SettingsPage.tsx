import { commands } from "@/bindings";
import { LanguageSelector, ThemeSelector } from "@/components";
import { Convert, Theme } from "@repo/theme";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

const handleFetchAllThemes = async () => {
  try {
    let response = await commands.fetchAllThemes();
    if (response.status === "ok") {
      return response.data;
    }
    throw new Error("Failed to fetch themes: Invalid response status");
  } catch (error) {
    console.error("Failed to fetch themes:", error);
    throw error;
  }
};

const handleReadTheme = async (themeName: string): Promise<Theme> => {
  try {
    let response = await commands.readTheme(themeName);
    if (response.status === "ok") {
      return Convert.toTheme(response.data);
    }
    throw new Error("Failed to read theme: Invalid response status");
  } catch (error) {
    console.error("Failed to read theme:", error);
    throw error;
  }
};

export const SettingsPage = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const { i18n } = useTranslation();

  const [number, setNumber] = useState<number>(0);
  const [constantValue, setConstantValue] = useState<number>(0);

  const [themes, setThemes] = useState<string[]>([]);
  const [selectedTheme, setSelectedTheme] = useState<Theme | undefined>(undefined);

  // Initialize theme
  useEffect(() => {
    const initializeThemes = async () => {
      try {
        const allThemes = await handleFetchAllThemes();
        if (allThemes) {
          setThemes(allThemes);
        }

        const savedThemeName = localStorage.getItem("theme");
        let themeToUse: Theme | undefined;

        if (savedThemeName) {
          themeToUse = await handleReadTheme(savedThemeName);
        }

        if (themeToUse) {
          setSelectedTheme(themeToUse);
        } else {
          localStorage.setItem("theme", themes[0]);
          setSelectedTheme(await handleReadTheme(themes[0]));
        }
      } catch (error) {
        console.error("Failed to initialize themes:", error);
      }
    };

    initializeThemes();
  }, []);

  // Initialize language
  useEffect(() => {
    const setLanguageFromLocalStorage = () => {
      const savedLanguage = localStorage.getItem("language");
      if (savedLanguage) {
        i18n.changeLanguage(savedLanguage);
      }
    };
    setLanguageFromLocalStorage();
  }, [i18n]);

  // Handle theme change
  useEffect(() => {
    const handleStorageChange = async () => {
      const storedTheme = localStorage.getItem("theme");
      if (storedTheme) {
        setSelectedTheme(await handleReadTheme(storedTheme));
      }
    };

    window.addEventListener("storage", handleStorageChange);

    return () => {
      window.removeEventListener("storage", handleStorageChange);
    };
  }, [themes]);

  useEffect(() => {
    if (!selectedTheme) {
      console.error("Failed to initialize theme");
    }
  }, [selectedTheme]);

  useEffect(() => {
    const unlisten = listen<number>("font-size-update-event", (event) => {
      setConstantValue(event.payload);
    });

    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, []);

  const handleButtonClick = async () => {
    try {
      const response = await commands.updateFontSize(number);
      if (response.status === "ok") {
        console.log("OK");
      }
    } catch (err) {
      console.error("Failed to update font size:", err);
    }
  };

  return (
    <main>
      <div>
        <h3>{t("selectLanguage")}</h3>
        <LanguageSelector />
      </div>
      <div>
        <h3>{t("selectTheme")}</h3>
        <ThemeSelector themes={themes} />
      </div>
      <div>
        <h3>Update Font Size</h3>
        <input type="number" value={number} onChange={(e) => setNumber(parseInt(e.target.value))} />
        <button onClick={handleButtonClick}>Update</button>
      </div>
      <div>
        <h3>Font Size</h3>
        <p>{constantValue}</p>
      </div>
    </main>
  );
};
