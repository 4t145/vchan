import * as NavigationMenu from "@radix-ui/react-navigation-menu";
import * as Label from "@radix-ui/react-label";
import { CopyIcon, TrashIcon } from "@radix-ui/react-icons";
import * as RadioGroup from "@radix-ui/react-radio-group";
import { api } from "../../webapi";

import { clsx } from "clsx";
import { useCookieTokens, useCopyToClipboard, CookieToken } from "../../hooks";

import { useState } from "react";
import { UserId } from "../UserId";
import { useCookies } from "react-cookie";
import { VcrButton } from "../VcrButton";

import { css, theme } from "../../../stitches.config";

const RadioGroupIndicator = css({
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  width: "100%",
  height: "100%",
  position: "relative",
  "&::after": {
    content: '""',
    display: "block",
    backgroundColor: "$vc_main",
    borderRadius: "4px",

    width: "1.5rem",
    height: "1.5rem",
  },
});

const RadioGroupItem = css({
  all: "unset",
  backgroundColor: "$vc_surface",
  width: "2rem",
  height: "2rem",
  borderRadius: "4px",
  boxShadow: `4px 4px ${theme.colors.vc_main}`,
  "&:hover": { backgroundColor: "$vc_main" },
});

const RadioGroupRoot = css({
  display: "flex",
  flexDirection: "column",
  gap: 10,
});

export function CookieBox() {
  const [cookieTokens, setCookieTokens] = useCookieTokens();
  const [_, copy] = useCopyToClipboard();
  const [cookies, setCookies] = useCookies();
  const currentCookie = CookieToken.parse(cookies['vchan_token'])?.id
  return (
    <>
      <RadioGroup.Root className={RadioGroupRoot()} defaultValue={currentCookie} onValueChange={
        (cookie_id) => {
          const ct = cookieTokens.find(ct => ct.id === cookie_id)!;
            setCookies("vchan_token", CookieToken.format(ct), {
              path: '/',
              expires: new Date(0),
              maxAge: Number.MAX_SAFE_INTEGER,
              secure: false,
              domain: 'localhost',
            });
        }
      }>
        {cookieTokens.length != 0 ? (
          cookieTokens.map((cookie) => (
            <div
              key={cookie.id}
              
              style={{ display: "flex", alignItems: "center", gap: "8px" }}
            >
              <RadioGroup.Item className={RadioGroupItem()} value={cookie.id}>
                <RadioGroup.Indicator className={RadioGroupIndicator()} />
              </RadioGroup.Item>
              <label style={{ flexGrow: 1, fontSize: "1.5rem" }}>
                <UserId user_id={cookie.id} />
              </label>
              {/* 复制到剪贴板 */}
              <VcrButton onClick={() => copy(CookieToken.format(cookie))}>
                <CopyIcon />
              </VcrButton>
              {/* 删除 */}
              <VcrButton>
                <TrashIcon onClick={() => {
                  const newCookieTokens = cookieTokens.filter(ct => ct.id!==cookie.id);
                  setCookieTokens(newCookieTokens)
                }}/>
              </VcrButton>
            </div>
          ))
        ) : (
          <tr className="text-black w-full flex justify-center">空空如也</tr>
        )}
      </RadioGroup.Root>

      <div
        style={{
          display: "flex",
          justifyContent: "right",
        }}
      >
        <VcrButton
          onClick={async () => {
            const registerResult = await api.register();
            if (registerResult.ok) {
              const cookie = CookieToken.parse(
                registerResult.data.cookie_token
              );
              if (cookie) setCookieTokens([...cookieTokens, cookie]);
              else; // for popup errors
            } else {
              // for popup errors
            }
          }}
        >
          从服务器获取
        </VcrButton>
        <VcrButton onClick={
          async () => {
            const code = await navigator.clipboard.readText()
            const ct = CookieToken.parse(code)
            if (ct != null) {
              setCookieTokens([...cookieTokens, ct]);
            } else {
              // warn 无效的tok
            }
          }
        }>从剪贴板导入</VcrButton>
      </div>
    </>
  );
}