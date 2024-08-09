import { useTranslation } from "react-i18next";
import { commands, SessionInfoDTO } from "../bindings";
import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import {
  Tooltip,
  CodeIcon,
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuIconWrapper,
  Icon,
  SearchIcon,
} from "../../../shared/ui/src";

export const Home: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [sessionInfo, setSessionInfo] = useState<SessionInfoDTO | null>(null);
  const [data, setData] = useState<number | null>(null);

  useEffect(() => {
    const unlisten = listen<number>("data-stream", (event) => {
      setData(event.payload);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    if (sessionInfo) {
      console.log("Session restored:", sessionInfo);
    }
  }, [sessionInfo]);

  const handleRestoreSession = async () => {
    try {
      let response = await commands.restoreSession(null);
      if (response.status === "ok") {
        setSessionInfo(response.data);
      }
    } catch (error) {
      console.error("Failed to restore session:", error);
    }
  };

  return (
    <main>
      <h1>{t("title")}</h1>

      {sessionInfo ? (
        <div>
          <p>Session: {sessionInfo.session.id}</p>
          <p>Project: {sessionInfo.project.source}</p>
        </div>
      ) : (
        <p>No session</p>
      )}

      <button className="bg-red-500" onClick={handleRestoreSession}>
        Restore Session
      </button>

      <span>{t("description.part1")}</span>
      <span>{t("description.part1", { ns: "ns2" })}</span>
      {data !== null && <p>Received data: {data}</p>}

      <div>
        <Tooltip label="Test">
          <CodeIcon className="fill-red-500 w-4" />
        </Tooltip>
      </div>

      <div>
        <DropdownMenu>
          <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>

          <DropdownMenuContent>
            <DropdownMenuItem>
              <DropdownMenuIconWrapper>
                <Icon>
                  <SearchIcon className="text-red-600" />
                </Icon>
              </DropdownMenuIconWrapper>
              Menu item 1
            </DropdownMenuItem>
            <DropdownMenuItem>
              <DropdownMenuLabel>Menu item 2</DropdownMenuLabel>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </main>
  );
};

export default Home;
