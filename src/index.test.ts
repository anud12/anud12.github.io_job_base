import {readFileSync} from "fs";
import {newGoogleSession} from "./api/implementation/GoogleSession";


it("reads from sheet", async () => {
  process.env.CLIENT_EMAIL = readFileSync(`${__dirname}/client_email`).toString();
  process.env.PRIVATE_KEY = readFileSync(`${__dirname}/private_key`).toString();
  const session = await newGoogleSession();
  const files = await session.intoDrive().findOneById("1BlHXnJJPuWu0_VDe-JAJzZ0XAR_MplEW45TnmdMeKfI");

  let result = await files.intoSheet().findBy().size(1).query();
  console.log(result);
  expect(result).toBeTruthy();
})

test("reads files from drive", async () => {
  process.env.CLIENT_EMAIL = readFileSync(`${__dirname}/client_email`).toString();
  process.env.PRIVATE_KEY = readFileSync(`${__dirname}/private_key`).toString();
  const globalFs = (await newGoogleSession()).intoDrive();
  const trash = await globalFs.findOneById("1NwJuSyQ4rFoI6I07vAwjmm62X3zuCNxv");
  const db = (await globalFs.findOneById("1BlHXnJJPuWu0_VDe-JAJzZ0XAR_MplEW45TnmdMeKfI")).intoSheet()
  const images = await globalFs.findOneById("1gfpLitPAZtWF5omk97dB3IXbf_leWXPp");

  const create = await globalFs.findOneById("11zqF7BKA-7hvcONuSmbU2RGHrxmLafNl");
  const result = await create.findByName("form-data.json");
  expect(result).toBeTruthy();
}, 1000000000)