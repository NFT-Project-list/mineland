import React from "react";
import { ChevronDoubleRightIcon } from "@heroicons/react/outline";
import { Button } from "../basic/Button";
import { convertToYocto, defaultGas } from "../../near/api";
import { SellItem } from "./SellItem";

export const Sidebar = ({
  currentUser,
  contract,
  sellList,
  setSellList,
  isOpen,
  setIsOpen,
}) => {
  const cancelItemSell = (token_id, nftType) => {
    sellList[nftType] = sellList[nftType].filter(
      (stone) => stone.token_id !== token_id
    );
    setSellList({ ...sellList });
  };

  const setItemPrice = (nft, price, nftType) => {
    sellList[nftType] = sellList[nftType].map((item) => {
      if (item.token_id === nft.token_id) {
        item.sale_price = price;
      }
      return item;
    });
    console.log(sellList);
    setSellList({ ...sellList });
  };

  const createSaleObject = (list) => {
    let sellObject = {};
    let isError = false;
    list.forEach((item) => {
      if (item.sale_price > 0) {
        sellObject[item.token_id] = convertToYocto(item.sale_price);
      } else {
        isError = true;
      }
    });
    return [isError, sellObject];
  };

  const sellMyItems = () => {
    if (sellList["stones"].length) {
      const [isError, sellObject] = createSaleObject(sellList["stones"]);
      if (!isError) {
        sellStoneItems(sellObject);
      } else {
        alert("Please, provide sale price for all Stones");
      }
    } else if (sellList["mines"].length) {
      const [isError, sellObject] = createSaleObject(sellList["mines"]);
      if (!isError) {
        sellMineItems(sellObject);
      } else {
        alert("Please, provide sale price for all Mines");
      }
    }
  };

  const sellStoneItems = async (sellObject) => {
    await contract.publish_stones_on_market(
      {
        token_price_list: sellObject,
        account_id: currentUser.accountId,
      },
      defaultGas,
      1
    );
  };

  const sellMineItems = async (sellObject) => {
    await contract.publish_mines_on_market(
      {
        token_price_list: sellObject,
        account_id: currentUser.accountId,
      },
      defaultGas,
      1
    );
  };

  const sellBtnText = () => {
    let result = `${sellList["mines"].length} Mine`;
    let type = "mines";

    if (sellList["mines"].length > 0) {
      result = `${sellList["mines"].length} Mine`;
      type = "mines";
    } else if (sellList["stones"].length > 0) {
      result = `${sellList["stones"].length} Stone`;
      type = "stones";
    }

    if (sellList[type].length > 1) {
      result += `s`;
    }

    return `Sell ${result}`;
  };

  const isSidebarEnabled = () => {
    return (
      sellList["stones"].length > 0 ||
      sellList["mines"].length > 0
    );
  };

  return (
    <>
      {isSidebarEnabled() && (
        <div
          className={`top-0 right-0 fixed w-[350px] h-full p-10 ease-in-out duration-300 bg-gray-800 z-30 
        shadow-3xl border-l-[4px] border-gray-600 ${
            isOpen ? "translate-x-0" : "translate-x-full"
          }`}
        >
          <div
            className={`bg-gray-800 px-2 w-10 h-14 absolute left-[-40px] bottom-9 cursor-pointer pt-3 shadow-3xl
            border-[4px] border-r-0  hover:text-indigo-100 
            ${
              isOpen
                ? "rounded-l-lg border-gray-600"
                : "rounded-r-lg rotate-180 border-gray-800"
            }`}
            onClick={() => setIsOpen(!isOpen)}
          >
            <ChevronDoubleRightIcon className="w-5 h-6 font-semibold" />
          </div>

          {Object.keys(sellList).map((key) => (
            <section key={key}>
              {sellList[key].length > 0 && (
                <div className="mb-10">
                  <h3 className="uppercase text-xl text-center font-semibold mb-4">
                    Sell {key}
                  </h3>
                  <div
                    className={`overflow-y-auto absolute bottom-32 top-24 right-10 left-10`}
                  >
                    {sellList[key].map((item) => (
                      <SellItem
                        key={item.token_id}
                        item_type={item.card_rarity || item.mine_type}
                        nft={item}
                        cancelSell={() => cancelItemSell(item.token_id, key)}
                        setItemPrice={setItemPrice}
                        id={key}
                      />
                    ))}
                  </div>
                </div>
              )}
            </section>
          ))}

          <div className="absolute bottom-10 text-center left-0 right-0">
            <Button title={sellBtnText()} noIcon onClick={sellMyItems} />
          </div>
        </div>
      )}
    </>
  );
};
