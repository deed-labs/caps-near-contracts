import { ExecutionContext } from "ava";

export const SOULBOUND_GAS = (200 * 1e3).toString() + "0".repeat(9);
export const SOULBOUND_CREATE_COST = (55 * 1e5).toString() + "0".repeat(18);
export const SOULBOUND_UPDATE_COST = "0";

export function failPromiseRejection(
    test: ExecutionContext,
    msg: string
): (e: any) => void {
    return (e: any) => {
        test.log(`Promise rejected while ${msg}:`);
        test.log(e);
        test.fail();
    };
}