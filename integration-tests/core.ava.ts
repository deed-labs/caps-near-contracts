import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";
import {DEPLOY_SOULBOUND_GAS, failPromiseRejection} from "./utils";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // deploy contract
  const root = worker.rootAccount;

  // FIXME: replace soul token account with actual deployed contract id
  const soulToken = await root.createSubAccount("soul", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const contract = await root.devDeploy(
    "./wasm/insoul_core.wasm",
    { method: "new", args: {soul_token_id: soulToken}, initialBalance: NEAR.parse("30 N").toJSON() }
  );

  // some test accounts
  const alice = await root.createSubAccount("alice", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const bob = await root.createSubAccount("bob", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const charlie = await root.createSubAccount("charlie", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, alice, bob, charlie };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("create soulbound", async (t) => {
  const { contract, alice } = t.context.accounts;
  await alice.call(contract, "create_soulbound",
      {
        metadata: { spec: "sbt-1.0", name: "john_snow", symbol: "JSSB" }
      },
      { gas: DEPLOY_SOULBOUND_GAS }
  ).catch(failPromiseRejection(t, "creating soulbound"));

});
