import React from "react";
import { CardBack } from "assets/styles/card";
import near_logo from "assets/images/near-logo.png";
import { formatId } from "utils/index";
import { Button } from "components/basic/Button";
import { ShoppingCartIcon } from "@heroicons/react/outline";
import mine_back from "assets/images/mine_back.png";
import { CardDropdown } from './CardDropdown';

export const BackCard = ({
  nft,
  size,
  sellItems,
  setSellItems,
  handleBuy,
  setSellPopupVisible,
  setTransferPopupVisible,
  noMenu,
  setKillItem
}) => {
  const sizeMapping = {
    xsm: "h-24",
    sm: "h-40",
    md: "h-80",
  };

  const BackImage = () => (
    <img
      className={`absolute ${sizeMapping[size || "md"]}`}
      src={mine_back}
      alt={nft.token_id ? formatId(nft.token_id) : ""}
    />
  );

  const SellPriceSection = () => (
    <>
      {nft.sale_price && (
        <div className="flex text-4xl items-center font-semibold mb-5 mt-2">
          <img className="h-8 mr-2" src={near_logo} alt="near logo" />
          {nft.sale_price}
        </div>
      )}
    </>
  );

  const InfoSection = () => (
    <>
      <div className="font-semibold text-gray-600">
        {nft.card_rarity || nft.mine_type}
      </div>
      <div className="text-4xl font-semibold text-gray-600">
        #{nft.token_id ? formatId(nft.token_id) : ""}
      </div>
    </>
  );

  const AdditionalInfoSection = () => (
    <>
      {nft.hardness && nft.density && nft.durability && (
        <div className="mt-6 text-gray-900">
          <p>Hardness: {nft.hardness}</p>
          <p>Density: {nft.density}</p>
          <p>Durability: {nft.durability}</p>
        </div>
      )}
    </>
  );

  const CardActions = () => (
    <>
      {sellItems && setSellItems && (
        <div className="mt-5 flex flex-col h-24 justify-between">
          <Button
            onClick={() => setTransferPopupVisible(true)}
            dark
            title="Transfer"
          />
          <Button
            onClick={() => setSellPopupVisible(true)}
            icon={<ShoppingCartIcon className="h-5 ml-1" />}
            title="Sell"
          />
        </div>
      )}

      {handleBuy && (
        <div className="mt-5">
          <Button
            onClick={handleBuy}
            icon={<ShoppingCartIcon className="h-5 ml-1" />}
            title="Buy"
          />
        </div>
      )}
    </>
  );

  return (
    <CardBack type={nft.mine_type ?? nft.card_rarity} className="relative">
      <BackImage />

      <div className="relative z-20 justify-end flex pt-3 mr-4">
        {nft.token_id && !noMenu && (
          <CardDropdown
            setTransferPopupVisible={setTransferPopupVisible}
            setSellItems={setSellItems}
            setKillItem={setKillItem}
          />
        )}
      </div>

      <div className="absolute flex flex-col h-full w-full justify-center items-center pb-20">
        <SellPriceSection />
        <InfoSection />
        <AdditionalInfoSection />
        <CardActions />
      </div>
    </CardBack>
  );
};
