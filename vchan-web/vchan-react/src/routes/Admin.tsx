import { VcrButton } from "../components";
import { fetchWebapi } from "../repo";
import { Suspense, useEffect, useState } from "react";
import { api, Board, VchanResponse } from "../webapi";
import React from "react";
import { LoadingWebData, useLoadingWebData } from "../models";
export function Admin() {
  return (
    <>
      <div>
        <BoardPanelAdmin />
      </div>
    </>
  );
}

function BoardPanelAdmin() {
  const resp = useLoadingWebData(() => api.get_board());
  if (resp.loading) {
    return <>加载中</>;
  } else {
    if (resp.success) {
      const result = resp.result;
      if (result.ok) {
        const boards = result.data;
        return (
          <>
            <input></input>
            <VcrButton>新建</VcrButton>
            <ul>
              {boards.map((b) => (
                <li key={b.id}>
                  <a href="">{b.name}</a>({b.id})
                </li>
              ))}
            </ul>
          </>
        );
      } else {
        const error = result.err;
        return <>{error.type}</>;
      }
    } else {
      const message = resp.message;
      return <>{message}</>;
    }
  }
}
