// FIXME: Trans component can also be used for translation
import { useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";

const isActive = ({ isActive }: any) => `link ${isActive ? "active" : ""}`;

export const Menu = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  return (
    <nav>
      <div>
        <NavLink className={isActive + " bg-green-300 text-primary px-20"} to="/">
          {t("home")}
        </NavLink>
        <NavLink className={isActive + " bg-orange-300 text-primary px-20"} to="/settings">
          {t("settings")}
        </NavLink>
        <NavLink className={isActive + " bg-yellow-300 text-primary px-20"} to="/logs">
          logs
        </NavLink>
        <NavLink className={isActive + " bg-sky-400 text-primary px-20"} to="/Dockview">
          Dockview
        </NavLink>
      </div>
    </nav>
  );
};
