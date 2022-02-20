import { connect, Contract, keyStores, WalletConnection } from "near-api-js";
import { getConfig } from "./config";
import Big from "big.js";

const nearConfig = getConfig(process.env.NODE_ENV || "development");

export const defaultGas = Big(60)
  .times(10 ** 12)
  .toFixed();

export const defaultDeposit = Big(0.01)
  .times(10 ** 24)
  .toFixed();

export const getMedia = (media) => `https://ipfs.io/ipfs/${media}`;

export const convertFromYocto = (amount, digits = 1) => {
  return Big(amount)
    .div(10 ** 24)
    .toFixed(digits);
};

export const convertFromNanoSeconds = (timestamp) => {
  return parseInt(Big(timestamp).div(1000000).toFixed());
};

export const convertToYocto = (amount) => {
  return Big(amount)
    .times(10 ** 24)
    .toFixed();
};
export const convertToTera = (amount) => {
  return Big(amount)
    .times(10 ** 12)
    .toFixed();
};

export async function initContract() {
  const near = await connect(
    Object.assign(
      { deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } },
      nearConfig
    )
  );

  window.walletConnection = new WalletConnection(near);
  window.accountId = window.walletConnection.getAccountId();

  // Initializing our contracts
  window.contract = await new Contract(
    window.walletConnection.account(),
    nearConfig.contractName,
    {
      viewMethods: [
        "user_mines",
        "user_stones",
        "total_mines_count",
        "get_collections",
        "get_one_collection",
        "user_collection_counts",
        "get_mines_from_market",
        "get_stones_from_market",
      ],
      changeMethods: [
        "mint_mine_nft",
        "mint_free_stone_nft",
        "publish_mines_on_market",
        "publish_stones_on_market",
        "mint_collection",
        "transfer_mine",
        "transfer_stone",
        "stone_kill"
      ],
    }
  );

  window.ftContract = await new Contract(
    window.walletConnection.account(),
    `ft.${nearConfig.contractName}`,
    {
      viewMethods: ["ft_balance_of"],
      changeMethods: [],
    }
  );
}

export function login() {
  window.walletConnection.requestSignIn(
    nearConfig.contractName,
    "",
    window.location.origin + "/mines"
  );
}

export function logout() {
  window.walletConnection.signOut();
  window.location.replace(window.location.origin);
}
