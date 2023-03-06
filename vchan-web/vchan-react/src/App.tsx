import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import PageContainer from "./components/PageContainer";
import { VcrButton } from "./components/VcrButton";
import { CookieIcon } from "@radix-ui/react-icons";
import { Route, Outlet  } from "react-router-dom";
function App() {
  return (
    <PageContainer>
      <Outlet ></Outlet>
    </PageContainer>
  );
}

export default App;
