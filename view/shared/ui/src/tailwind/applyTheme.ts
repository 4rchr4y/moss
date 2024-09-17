import { Theme, ThemeCssVariables, mapThemeToCssVariables } from "@repo/theme";

//Applies the theme to the root element of the app.
export default function applyTheme(theme: Theme) {
  const themeObject: ThemeCssVariables = mapThemeToCssVariables(theme);
  const root = document.documentElement;

  Object.keys(themeObject).forEach((v) => {
    const propertyVal = themeObject[v as keyof ThemeCssVariables];
    if (propertyVal) root.style.setProperty(v, propertyVal);
  });
}
