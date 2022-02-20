import React, { useEffect, useState } from "react";
import {
  Container,
  InnerPageWrapper,
  Wrapper,
} from "assets/styles/common.style";
import { Header } from "components/Header";
import { Button } from "components/basic/Button";
import { Footer } from "components/Footer";
import { InnerPageHead } from "components/InnerPageHead";
import { MineContent } from "utils/content";
import { Loader } from "components/basic/Loader";
import { Popup } from "components/Popup";
import { MintMineSection } from "./MintMineSection";
import { Card } from "../../components/card/Card";
import { convertFromYocto, convertToTera } from "../../near/api";

export const Mines = ({ currentUser, contract, sellList, setSellList }) => {
  const [allMines, setAllMines] = useState({});
  const [userMines, setUserMines] = useState([]);
  const [mintPopupVisible, setMintPopupVisible] = useState(false);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    const userMinesPromise = new Promise(async (resolve, reject) => {
      const userMines = await contract
        .user_mines({
          account_id: currentUser.accountId,
        })
        .catch((err) => reject(err));
      resolve(userMines);
    });

    const allMinesPromise = new Promise(async (resolve, reject) => {
      const allMines = await contract
        .total_mines_count()
        .catch((err) => reject(err));
      resolve(allMines);
    });

    Promise.all([userMinesPromise, allMinesPromise]).then((result) => {
      // Convert price from Yocto NEAR
      const mines = result[0].map((ln) => {
        if (ln.sale_price) {
          ln.sale_price = convertFromYocto(ln.sale_price);
        }
        return ln;
      });
      setUserMines(mines);
      setAllMines(result[1]);
      setIsReady(true);
    });
  }, []);

  const handleTransfer = async (mine, transferAddress) => {
    let gas = convertToTera("60");
    await contract.transfer_mine(
      {
        token_id: mine.token_id,
        recipient_id: transferAddress,
      },
      gas,
      1
    );
  };

  const appendToSellList = (mine) => {
    if (
      !sellList["mines"].filter((exist) => exist.token_id === mine.token_id)
        .length
    ) {
      sellList["mines"].push(mine);
      sellList["stones"] = [];
      setSellList({ ...sellList });
    }
  };

  const showMintPopup = async () => {
    setMintPopupVisible(true);
  };

  return (
    <InnerPageWrapper>
      <Header currentUser={currentUser} />

      <Wrapper>
        <Container className="text-white text-center mt-8">
          <InnerPageHead
            title={MineContent.title}
            description={MineContent.description}
          />

          {!userMines.length || (
            <Button
              title="Buy More Mines"
              size="lg"
              animated
              noIcon
              onClick={showMintPopup}
            />
          )}

          <div className="mt-10 pb-16 w-full">
            {isReady ? (
              <div className="lg:flex items-center justify-center w-full flex-wrap gap-6">
                {userMines.length ? (
                  userMines.map((mine, index) => (
                    <Card
                      nft={mine}
                      key={index}
                      contract={contract}
                      currentUser={currentUser}
                      sellItems={sellList["mines"]}
                      setSellItems={() => appendToSellList(mine)}
                      handleTransfer={(transferAddress) =>
                        handleTransfer(mine, transferAddress)
                      }
                    />
                  ))
                ) : (
                  <div>
                    <div className="mb-7 leading-10">
                      <b className="text-xl">{MineContent.no_mines}.</b> <br />
                      <p className="text-cyan-200 leading-6">
                        {MineContent.no_mines_details}:
                      </p>
                    </div>
                    <MintMineSection
                      currentUser={currentUser}
                      contract={contract}
                      allMines={allMines}
                      minesCount={userMines.length}
                    />
                  </div>
                )}
              </div>
            ) : (
              <Loader />
            )}
          </div>
        </Container>

        <Popup
          title="Buy More Mines"
          popupVisible={mintPopupVisible}
          setPopupVisible={setMintPopupVisible}
        >
          <div className="mt-2">
            <MintMineSection
              currentUser={currentUser}
              contract={contract}
              allMines={allMines}
              minesCount={userMines.length}
            />
          </div>
        </Popup>
      </Wrapper>
      <Footer />
    </InnerPageWrapper>
  );
};
