import React, { useEffect, useState } from "react";
import {
  convertFromNanoSeconds,
  convertFromYocto,
  convertToTera,
  convertToYocto,
} from "near/api";
import {
  Container,
  InnerPageWrapper,
  Wrapper,
} from "assets/styles/common.style";
import { MineContent, StoneContent } from "utils/content";
import { Header } from "components/Header";
import { Footer } from "components/Footer";
import { InnerPageHead } from "components/InnerPageHead";
import { Button } from "components/basic/Button";
import MintStonePopup from "./MintStonePopup";
import { Card } from "components/card/Card";
import { Loader } from "components/basic/Loader";
import Dropdown from "components/basic/Dropdown";
import { Pagination } from "components/Pagination";
import { useLocation, useNavigate } from "react-router-dom";
import { Popup } from "../../components/Popup";
import { formatId } from '../../utils';

const PAGE_LIMIT = "40";

export const Stones = ({ currentUser, contract, sellList, setSellList }) => {
  const [isReady, setIsReady] = useState(false);
  const [userStones, setUserStones] = useState([0, []]); // [<count>, [<arrayOfStones>]]
  const [currentPage, setCurrentPage] = useState(1);
  const [userMines, setUserMines] = useState([]);
  const [userClaimCount, setUserClaimCount] = useState(0);
  const [mintPopupVisible, setMintPopupVisible] = useState(false);
  const [killPopupVisible, setKillPopupVisible] = useState(false);
  const [killItem, setKillItem] = useState(null);
  const [filterRarity, setFilterRarity] = useState(null);
  const [filterCollection, setFilterCollection] = useState(null);
  const [allCollections, setAllCollections] = useState([]);

  const navigate = useNavigate();
  const location = useLocation();

  async function fetchUserStones(currentPage) {
    let requestParams = {
      account_id: currentUser.accountId,
      page_num: currentPage.toString(),
      page_limit: PAGE_LIMIT,
    };
    if (filterCollection) {
      requestParams["filter_collection"] = Number(filterCollection);
    }
    if (filterRarity) {
      requestParams["filter_rarity"] = filterRarity;
    }
    let stones = await contract.user_stones(requestParams);

    // Convert price from Yocto NEAR
    stones[1] = stones[1].map((zm) => {
      if (zm.sale_price) {
        zm.sale_price = convertFromYocto(zm.sale_price);
      }
      return zm;
    });

    setUserStones(stones);
    setIsReady(true);
  }

  async function fetchCollections() {
    setAllCollections(await contract.get_collections());
  }

  const appendToSellList = (stone) => {
    if (
      !sellList["stones"].filter((exist) => exist.token_id === stone.token_id)
        .length
    ) {
      sellList["stones"].push(stone);
      sellList["mines"] = [];
      setSellList({ ...sellList });
    }
  };

  const buildUrl = () => {
    let url = `/stones?page=${currentPage}`;
    if (filterRarity) url = `${url}&rarity=${filterRarity}`;
    if (filterCollection) url = `${url}&collection=${filterCollection}`;

    return url;
  };

  async function fetchUserMines() {
    let timeNow = new Date().getTime();
    let oneDay = 24 * 60 * 60 * 1000;
    let userMines = await contract.user_mines({
      account_id: currentUser.accountId,
    });

    let totalStonesToMint = 0;
    userMines = userMines.map((mine) => {
      const lastClaimTime = convertFromNanoSeconds(mine.last_stone_claim);
      if (!lastClaimTime || timeNow - lastClaimTime > oneDay) {
        mine.can_claim = true;
        if (mine.mine_type === "Small") {
          totalStonesToMint += 1;
        } else if (mine.mine_type === "Medium") {
          totalStonesToMint += 4;
        } else {
          totalStonesToMint += 8;
        }
      } else {
        mine.can_claim = false;
      }
      return mine;
    });

    setUserClaimCount(totalStonesToMint);
    setUserMines(userMines);
  }

  useEffect(() => {
    const searchParams = new URLSearchParams(location.search);
    const page = JSON.parse(searchParams.has("page"))
      ? searchParams.get("page")
      : currentPage;
    const rarity = JSON.parse(searchParams.has("rarity"))
      ? searchParams.get("rarity")
      : filterRarity;
    const collection = JSON.parse(searchParams.has("collection"))
      ? searchParams.get("collection")
      : filterCollection;

    setCurrentPage(page);
    setFilterRarity(rarity);
    setFilterCollection(collection);

    fetchUserMines();
    fetchCollections();
    fetchUserStones(page);
  }, []);

  useEffect(() => {
    setCurrentPage(1);
    fetchCollections();
    fetchUserStones(1);
    navigate(buildUrl());
  }, [filterRarity, filterCollection]);

  useEffect(() => navigate(buildUrl()), [currentPage]);

  const handleMint = async (mineId, mineType) => {
    let gas;
    let deposit;

    if (mineType === "Small") {
      gas = convertToTera("50");
      deposit = convertToYocto("0.01");
    } else if (mineType === "Medium") {
      gas = convertToTera("120");
      deposit = convertToYocto("0.03");
    } else {
      gas = convertToTera("200");
      deposit = convertToYocto("0.06");
    }

    let newStones = await contract.mint_free_stone_nft(
      { mine_id: mineId },
      gas,
      deposit
    );
    setUserStones([...userStones, ...newStones]);
  };

  const showMintStonesBlock = () => {
    setMintPopupVisible(true);
  };

  const handleTransfer = async (stone, transferAddress) => {
    let gas = convertToTera("60");
    await contract.transfer_stone({
      token_id: stone.token_id,
      recipient_id: transferAddress
    }, gas, 1);
  };

  const rarityOptions = () => {
    return [
      {
        title: "All",
        onClick: () => setFilterRarity(null),
      },
      {
        title: "Common",
        onClick: () => setFilterRarity("Common"),
      },
      {
        title: "UnCommon",
        onClick: () => setFilterRarity("UnCommon"),
      },
      {
        title: "Rare",
        onClick: () => setFilterRarity("Rare"),
      },
      {
        title: "Legendary",
        onClick: () => setFilterRarity("Legendary"),
      },
    ];
  };

  const collectionOptions = () => {
    const collections = Object.keys(allCollections).map((key) => {
      return {
        title: allCollections[key].title,
        onClick: () => setFilterCollection(key),
      };
    });
    return [
      {
        title: "All",
        onClick: () => setFilterCollection(null),
      },
      ...collections,
    ];
  };

  const onPageChanged = (page) => {
    window.scrollTo({ top: 0, behavior: "smooth" });

    setCurrentPage(page);
    fetchUserStones(page);
  };

  const showKillPopup = (item) => {
    setKillItem(item);
    setKillPopupVisible(true);
  };

  const handleKill = async () => {
    let gas = convertToTera("90");
    await contract.stone_kill({
      stone_id: killItem.token_id,
    }, gas, 1);
  };

  return (
    <InnerPageWrapper>
      <Header currentUser={currentUser} />

      <Wrapper>
        <Container className="text-white text-center mt-8">
          <InnerPageHead
            title={StoneContent.title}
            description={StoneContent.description}
          />

          {isReady ? (
            <>
              {userMines.length ? (
                <div className="flex justify-between">
                  <div className="basis-4/12 text-left pt-4">
                    Total:
                    <span className="font-semibold ml-1">
                      {userStones[0]} NFTs
                    </span>
                  </div>
                  <Button
                    title={`Mint ${
                      userClaimCount > 0 ? userClaimCount : ""
                    } Stone${userClaimCount !== 1 ? "s" : ""}`}
                    size="lg"
                    noIcon
                    disabled={userClaimCount === 0}
                    onClick={showMintStonesBlock}
                  />
                  <div className="basis-4/12 z-10 text-right">
                    <div className="inline-block mr-3">
                      <Dropdown
                        title="Rarity"
                        selected={filterRarity}
                        options={rarityOptions()}
                      />
                    </div>
                    <div className="inline-block">
                      <Dropdown
                        title="Collection"
                        selected={
                          filterCollection
                            ? allCollections[filterCollection]?.title
                            : null
                        }
                        options={collectionOptions()}
                      />
                    </div>
                  </div>
                </div>
              ) : (
                <div className="mb-7 mt-10 leading-10">
                  <b className="text-xl">{MineContent.no_mines}.</b> <br />
                  <p className="text-cyan-200 w-1/2 px-16 mx-auto leading-6">
                    {StoneContent.no_mines_details}
                  </p>
                </div>
              )}

              <div className="mt-10 pb-16 w-full">
                {userMines.length &&
                (userStones[0] > 0 ? (
                  <div className="lg:flex flex-wrap items-center justify-center w-full gap-6">
                    {userStones[1]?.map((stone, index) => (
                      <Card
                        nft={stone}
                        key={index}
                        sellItems={sellList['stones']}
                        setSellItems={() => appendToSellList(stone)}
                        handleTransfer={(transferAddress) =>
                          handleTransfer(stone, transferAddress)
                        }
                        setKillItem={() => showKillPopup(stone)}
                      />
                    ))}
                  </div>
                ) : (
                  <div>
                    You don't have{" "}
                    <span className="ml-1 mr-2">{filterRarity}</span>
                    {filterCollection
                      ? allCollections[filterCollection].title
                      : ""}{" "}
                    Stones.
                  </div>
                ))}
              </div>

              <div className="mb-8">
                <Pagination
                  total={parseInt(userStones[0])}
                  limit={parseInt(PAGE_LIMIT)}
                  selectedPage={currentPage}
                  onPageChanged={onPageChanged}
                />
              </div>
            </>
          ) : (
            <Loader />
          )}
        </Container>

        <MintStonePopup
          mintPopupVisible={mintPopupVisible}
          setMintPopupVisible={setMintPopupVisible}
          userMines={userMines}
          handleMint={handleMint}
        />

        <Popup
          title="Destroy Stone"
          popupVisible={killPopupVisible}
          setPopupVisible={setKillPopupVisible}
        >
          {killItem && (
            <div className="mt-2">
              <p className="mb-6">Stone {" "}
                <span className="text-xl font-semibold">#{formatId(killItem.token_id)}</span>{" "}
                will be removed and you will receive{" "}
                <span className="text-xl font-semibold">{convertFromYocto(killItem.kill_tokens)} MNL</span> tokens.
              </p>

              <div className="mr-3 inline-block">
                <Button title="Cancel" secondary noIcon onClick={() => setKillPopupVisible(false)} />
              </div>
              <div className="inline-block">
                <Button title="Exchange to MLN" onClick={handleKill} />
              </div>
            </div>
          )}

        </Popup>

      </Wrapper>

      <Footer />
    </InnerPageWrapper>
  );
};
