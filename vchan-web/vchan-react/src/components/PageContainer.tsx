import { HomeMenu } from "../components";
import { CookieBox } from "../components/CookieBox";
import { Cross1Icon, CookieIcon } from "@radix-ui/react-icons";
import * as Dialog from "@radix-ui/react-dialog";
import clsx from "clsx";
import { ReactNode, useState } from "react";
import { VchanDialog } from "./VchanDialog";
import { VcrButton } from "./VcrButton";
import { styled, css, theme } from "../../stitches.config";

const NavBar = css({
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  backgroundColor: "$vc_surface",
  width: "100vw",
  height: "56px",
  borderBottom: `solid 4px ${theme.colors.vc_main}`
});

const Menu = css({
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  backgroundColor: "$vc_surface",
  width: "100vw",
  height: "56px",
});

const MainContainer = css({
  display: "flex",
  flexDirection: "row",
  justifyContent: "space-between",
  backgroundColor: "$vc_dark",
  width: "100vw",
  height: "100%",
});

const CenterPart = css({
  display: "flex",
  flexDirection: "column",
  justifyContent: "flexStart",
  backgroundColor: "$vc_dark",
  flexGrow: 1,
  color: "White",
  padding: "1rem 3rem 1rem 3rem",
  maxWidth: "1080px"
});

export default function PageContainer(props: React.PropsWithChildren<{}>) {
  return (
    <>
      <nav className={NavBar()}>
        <div
          className={css({
            paddingLeft: "2em",
            paddingRight: "2em",
            display: "flex",
            flexGrow: 1,
          })()}
        >
          vChan
        </div>
        <div
          className={css({
            paddingLeft: "2em",
            paddingRight: "2em",
            display: "flex",
            flexGrow: 0,
          })()}
        >
          <VchanDialog
            title="cookie面板"
            trigger={
              <CookieIcon />
            }
          >
            <CookieBox />
          </VchanDialog>
        </div>
      </nav>
      <main className={MainContainer()}>
        <aside>
          <HomeMenu />
        </aside>
        <div className={CenterPart()}>
          {props.children}
        </div>
        <aside>
          left
        </aside>
      </main>
      <footer></footer>
    </>
  );
}
