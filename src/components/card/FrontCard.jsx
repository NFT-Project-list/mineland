import React from "react";
import { getMedia } from "near/api";
import { CardFront, Rarity } from "assets/styles/card";
import { formatId } from "utils/index";
import { Price } from "components/Price";
import { CardDropdown } from "./CardDropdown";

export const FrontCard = ({
  nft,
  size,
  noMenu,
  setSellItems,
  setTransferPopupVisible,
  setKillItem
}) => {
  const type = nft.mine_type ?? nft.card_rarity;

  return (
    <CardFront type={type}>
      <img
        className={`absolute max-w-full ${size !== "sm" ? "h-80" : ""}`}
        src={getMedia(nft.media)}
        alt={nft.token_id ? nft.token_id : ""}
      />
      <div className="absolute flex w-full">
        <div className="flex w-full p-2 items-center justify-between">
          <Rarity
            type={type}
            className={`${size !== "sm" ? "pl-2" : "text-sm pl-1"}`}
          >
            {type}
          </Rarity>
          {nft.sale_price && size !== "sm" && <Price title={nft.sale_price} />}

          {nft.token_id && !noMenu && (
            <CardDropdown
              setTransferPopupVisible={setTransferPopupVisible}
              setSellItems={setSellItems}
              setKillItem={setKillItem}
            />
          )}
        </div>
      </div>
      {nft.token_id && (
        <div
          className={`absolute flex font-semibold justify-center w-full ${
            size === "sm" ? "bottom-1 text-base" : "bottom-3 text-2xl"
          }`}
        >
          #{formatId(nft.token_id)}
        </div>
      )}
    </CardFront>
  );
};
