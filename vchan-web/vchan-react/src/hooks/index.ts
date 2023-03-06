import { useCookies } from "react-cookie";
import { memo, useContext, useMemo, useState } from "react";
import { isEmpty } from "lodash";
import { Board } from "../webapi";

type CopiedValue = string | null;

type CopyFn = (text: string) => Promise<boolean>; // Return success

export function useCopyToClipboard(): [CopiedValue, CopyFn] {
  const [copiedText, setCopiedText] = useState<CopiedValue>(null);

  const copy: CopyFn = async (text) => {
    if (!navigator?.clipboard) {
      //   console.warn("Clipboard not supported");
      return false;
    }

    try {
      await navigator.clipboard.writeText(text);

      setCopiedText(text);

      return true;
    } catch (error) {
      //   console.warn("Copy failed", error);

      setCopiedText(null);

      return false;
    }
  };

  return [copiedText, copy];
}


export type CookieToken = {
  id: string;
  seed: string;
};

export namespace CookieToken {
  const regex = /\[\d+\]([a-f|A-F|0-9]*)@(.*)/;
  export function format(cookieToken: CookieToken): string {
    return `[${cookieToken.seed.length}]${cookieToken.seed}@${cookieToken.id}`;
  }
  export function parse(s: string): null | CookieToken {
    const result = regex.exec(s);
    if (!result) return null;
    const [_, seed, id] = result;
    if (isEmpty(seed) || isEmpty(id)) return null;
    return {
      seed,
      id,
    };
  }
}

export function useCookieTokens(): [CookieToken[], (_: CookieToken[]) => void] {
  let localCookieTokens = JSON.parse(
    localStorage.getItem("cookies") ?? "[]"
  ) as CookieToken[];
  const [cookieTokens, setCookieTokensInner] = useState(localCookieTokens);
  // const cookies[]
  const setCookieTokens = (newCookieTokens: CookieToken[]) => {
    localStorage.setItem("cookies", JSON.stringify(newCookieTokens));
    setCookieTokensInner(newCookieTokens);
  };

  return [cookieTokens, setCookieTokens];
}