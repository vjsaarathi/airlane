import { useState } from "react";
import { Button } from "./ui/button";
import { Moon, Sun } from "lucide-react";


const ThemeSwitcher = () => {
  const [theme, setTheme] = useState("dark");
  const toggleTheme = () => {
    const next = "dark" === theme ? "light" : "dark";
    setTheme(next);
    document.getElementById("root")?.classList.remove("dark", "light")
    document.getElementById("root")?.classList.add(next);
  }
  return (
    <>
      <Button variant="outline" size="icon" className="size-8" onClick={() => toggleTheme()}>
        {theme === "dark" ? <Sun /> : <Moon />}
      </Button>
    </>
  )
}

export default ThemeSwitcher;
