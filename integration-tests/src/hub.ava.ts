import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";
import {SOULBOUND_GAS, failPromiseRejection, SOULBOUND_COST} from "./utils";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // deploy contract
  const root = worker.rootAccount;

  const hub = await root.devDeploy(
    "../wasm/hub.wasm",
    { method: "new", args: {}, initialBalance: NEAR.parse("100 N").toJSON() }
  );

  // some test accounts
  const alice = await root.createSubAccount("alice", {
    initialBalance: NEAR.parse("500 N").toJSON(),
  });
  const bob = await root.createSubAccount("bob", {
    initialBalance: NEAR.parse("500 N").toJSON(),
  });
  const charlie = await root.createSubAccount("charlie", {
    initialBalance: NEAR.parse("500 N").toJSON(),
  });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, hub, alice, bob, charlie };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("create soulbound", async (t) => {
  const { hub, alice } = t.context.accounts;

  await alice.call(hub, "create_soulbound",
      {
        metadata: { spec: "nft-1.0.0", name: "john_snow", symbol: "JSSB" }
      },
      { attachedDeposit: SOULBOUND_COST, gas: SOULBOUND_GAS }
  ).catch(failPromiseRejection(t, "creating alice soulbound"));
});

test("update soulbound", async (t) => {
  const { hub, bob, root } = t.context.accounts;

  await bob.call(hub, "create_soulbound",
      {
        metadata: { spec: "nft-1.0.0", name: "bob", symbol: "BOB" }
      },
      { attachedDeposit: SOULBOUND_COST, gas: SOULBOUND_GAS }
  ).catch(failPromiseRejection(t, "creating bob soulbound"));

  await bob.call(hub, "update_soulbound",
      {
        metadata: { spec: "nft-1.0.0", name: "john_snow", symbol: "JSSB", bio: "born Aegon Targaryen, is the son of Lyanna Stark and Rhaegar Targaryen, the late Prince of Dragonstone." }
      },
      { attachedDeposit: SOULBOUND_COST, gas: SOULBOUND_GAS }
  ).catch(failPromiseRejection(t, "updating bob soulbound"));

  const profile = await root.getAccount(`bob.${hub.accountId}`);
})