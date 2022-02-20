import { XIcon } from "@heroicons/react/outline";
import { getMedia } from "near/api";
import React from "react";
import { formatId, statusColorBorderMap } from "utils";

export const SellItem = ({ item_type, nft, cancelSell, setItemPrice, id }) => {
  const [priceInput, setPriceInput] = React.useState();

  return (
    <div className="flex flex-row mb-1 pb-1 relative border-b border-dashed border-gray-700 last:border-transparent">
      <img
        className={`w-14 border-2 rounded-md mr-4 ${statusColorBorderMap(
          item_type
        )}`}
        src={getMedia(nft.media)}
        alt={nft.token_id}
      />
      <div>
        <div className="font-semibold pt-2 mb-1">
          <span className="text-indigo-100 mr-1">{item_type}</span> #
          {formatId(nft.token_id)}
        </div>
        <div>
          <input
            type="number"
            min="0.01"
            max="10000"
            step="0.01"
            placeholder="Price"
            value={priceInput}
            onChange={(e) => {
              const price = e.target.value;
              setPriceInput(price);
              setItemPrice(nft, price, id);
            }}
            className="w-20 px-2 py-1 rounded-md bg-transparent border-2 border-gray-600 mr-1"
          />{" "}
          NEAR
        </div>
      </div>
      <div className="absolute right-0 top-5">
        <XIcon
          className="h-6 w-6 fill-red-500 hover:fill-red-700 cursor-pointer transition duration-200"
          onClick={cancelSell}
        />
      </div>
    </div>
  );
};
