import { ExecutionContext } from "ava";

export const SOULBOUND_GAS = (70 * 1e3).toString() + "0".repeat(9);

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