import React, { useEffect } from "react";
import { VchanResponse } from "../webapi";

export * from "./post";

export type LoadingWebData<T> =
  | {
      loading: true;
    }
  | {
      loading: false;
      success: true;
      result: VchanResponse<T>;
    }
  | {
      loading: false;
      success: false;
      message: string;
    };

export function useLoadingWebData<T>(loader: () => Promise<VchanResponse<T>>, deps?: React.DependencyList): LoadingWebData<T>  {
  const [data, setData] = React.useState<LoadingWebData<T>>({
    loading: true,
  });
  useEffect(() => {
    loader().then(
      (result) => {
        setData({
          loading: false,
          success: true,
          result,
        });
      },
      (err) => {
        setData({
          loading: false,
          success: false,
          message: String(err),
        });
      }
    );
    return () => {
        setData({
            loading: true,
        })
    }
  }, deps);
  return data
}
