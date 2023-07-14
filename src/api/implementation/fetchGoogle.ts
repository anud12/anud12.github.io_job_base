import fetch from "node-fetch";

export const fetchGoogle = <T = any>(...args:Parameters<typeof fetch>):Promise<T> => {
  return new Promise(async (resolve, reject) => {
    const response = await fetch(args[0], {
      ...args[1],
    });
    setTimeout(async () => {
      if(response.status !== 200) {
        reject(Error(`${response.status} ${response.statusText}: ${await response.text()}`));
        return;
      }
      resolve(response);
    }, 500);
  })
}