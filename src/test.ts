import { readFileSync } from "fs";
import { newGoogleSession } from "./api/implementation/GoogleSession";

process.env.CLIENT_EMAIL = readFileSync(`${__dirname}/client_email`).toString();
process.env.PRIVATE_KEY = readFileSync(`${__dirname}/private_key`).toString();
(async () => {
  const session = await newGoogleSession();
  const files = await session.intoDrive().findOneById("1BlHXnJJPuWu0_VDe-JAJzZ0XAR_MplEW45TnmdMeKfI");

  let result = await files.intoSheet().find_by().size(1).query();
  console.log(result);
})()
