import {readFileSync} from "fs";
import {newGoogleSession} from "./api/implementation/GoogleSession";

process.env.CLIENT_EMAIL =  readFileSync(`${__dirname}/client_email`).toString();
process.env.PRIVATE_KEY = readFileSync(`${__dirname}/private_key`).toString();
(async () => {
  console.log(await newGoogleSession())
})()
