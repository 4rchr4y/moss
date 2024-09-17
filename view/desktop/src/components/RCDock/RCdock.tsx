import { DockLayout, LayoutData } from "rc-dock";
import "./rc-dock.css";
import * as Pages from "../../pages/index";
import { useEffect } from "react";

const defaultLayout: LayoutData = {
  dockbox: {
    mode: "horizontal",

    children: [
      {
        tabs: [
          { id: "tab1", title: "tab1", content: <Pages.HomePage /> },
          { id: "tab2", title: "tab2", content: <Pages.SettingsPage /> },
          { id: "tab3", title: "tab3", content: <Pages.LogsPage /> },
        ],
        panelLock: { panelStyle: "main" },
      },
    ],
  },
};

const RCDock = () => {
  useEffect(() => {
    console.log("RCDock rerendered");
  });
  return (
    <DockLayout
      defaultLayout={defaultLayout}
      style={{
        position: "absolute",
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
      }}
      dropMode="edge"
      onLayoutChange={(layout) => {
        console.log("RCDock layout changed", layout);
      }}
    />
  );
};

export default RCDock;
