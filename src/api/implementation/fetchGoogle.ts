import fetch, {Response} from "node-fetch";

export const fetchGoogle = (...args:Parameters<typeof fetch>):Promise<Response> => {
  return new Promise(async (resolve, reject) => {
    const responsePromise = fetch(args[0], {
      ...args[1],
    });
    setTimeout(async () => {
      const response = await responsePromise
      if(response.status !== 200) {
        reject(Error(`${response.status} ${response.statusText}: ${await response.text()}`));
        return;
      }
      resolve(response);
    }, 100);
  })
}
