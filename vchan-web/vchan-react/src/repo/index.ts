import { VchanResponse, VchanError } from "../webapi";

export type Data<P> = P extends Promise<VchanResponse<infer T>> ? T : never;
enum Status {
  Pending = 0,
  Error = 1,
  Success = 2,
}
export function fetchWebapi<P extends Promise<VchanResponse<any>>>(p: P) {
  let status = Status.Pending;
  let result: Data<P> | VchanError;
  let suspender = p.then(
    (res) => {
      if (res.ok) {
        status = Status.Success;
        result = res.data;
      } else {
        status = Status.Error;
        result = res.err;
      }
    },
    (e) => {
      status = Status.Error;
      result = e;
    }
  );
  return () => {
    if (status === Status.Pending) {
      throw suspender;
    } else if (status === Status.Error) {
      throw result;
    } else {
      return result as Data<P>;
    }
  };
}
