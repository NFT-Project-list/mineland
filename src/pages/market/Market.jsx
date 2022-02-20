import React, { useEffect, useState } from "react";
import {
  Container,
  InnerPageWrapper,
  Wrapper,
} from "assets/styles/common.style";
import { MarketContent } from "utils/content";
import { InnerPageHead } from "components/InnerPageHead";
import { Header } from "components/Header";
import { Footer } from "components/Footer";
import { Loader } from "components/basic/Loader";
import { ButtonGroup } from "components/ButtonGroup";
import { Card } from "components/card/Card";
import { convertFromYocto } from "../../near/api";

const START = 0;
const LIMIT = 10;

export const Market = ({ currentUser, contract }) => {
  const [isReady, setIsReady] = useState(false);
  const [items, setItems] = useState([]);
  const [active, setActive] = useState("Mines");

  const getMines = async () => {
    setIsReady(false);
    let items = await contract
      .get_mines_from_market({
        start: START,
        limit: LIMIT,
      })
      .catch((err) => console.log(err));

    items = items.map((item) => {
      item.sale_price = convertFromYocto(item.sale_price);
      return item;
    });

    setItems(items);
    setIsReady(true);
    setActive("Mines");
  };

  const getStones = async () => {
    setIsReady(false);
    let items = await contract
      .get_stones_from_market({
        start: START,
        limit: LIMIT,
      })
      .catch((err) => console.log(err));

    items = items.map((item) => {
      item.sale_price = convertFromYocto(item.sale_price);
      return item;
    });

    setItems(items);
    setIsReady(true);
    setActive("Stones");
  };

  useEffect(() => {
    getMines();
  }, []);

  const handleBuy = () => {
    setIsReady(false);
    setIsReady(true);
  };

  return (
    <InnerPageWrapper>
      <Header currentUser={currentUser} />

      <Wrapper>
        <Container className="text-white text-center mt-8">
          <InnerPageHead
            title={MarketContent.title}
            description={MarketContent.description}
          />
          <div className="mb-10 w-full">
            <ButtonGroup
              items={[
                {
                  title: "Mines",
                  onClick: () => getMines(),
                  active: active === "Mines",
                },
                {
                  title: "Stones",
                  onClick: () => getStones(),
                  active: active === "Stones",
                },
              ]}
            />

            {isReady ? (
              <div className="lg:flex items-center justify-center w-full flex-wrap gap-6">
                {items.length &&
                items.map((item, index) => (
                  <Card
                    nft={item}
                    key={index}
                    contract={contract}
                    currentUser={currentUser}
                    handleBuy={handleBuy}
                  />
                ))}
              </div>
            ) : (
              <Loader />
            )}
          </div>
        </Container>
      </Wrapper>

      <Footer />
    </InnerPageWrapper>
  );
};
