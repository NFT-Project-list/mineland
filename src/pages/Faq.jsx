import React, { useState } from "react";
import {
  Col,
  Container,
  InnerPageWrapper,
  Row,
  Wrapper,
} from "assets/styles/common.style";
import { Header } from "components/Header";
import { Footer } from "components/Footer";
import { InnerPageHead } from "components/InnerPageHead";
import { FAQContent } from "utils/content";

const FaqQuestion = ({ title, children, index, openedIndex, changeOpened }) => (
  <Row>
    <Col
      className="border-2 border-sky-800 rounded-xl px-10 py-8 mb-4 text-left bg-main/80 relative"
      onClick={() => changeOpened(index)}
    >
      <h3 className="uppercase font-semibold text-2xl">{title}</h3>
      <div
        className={`leading-7 overflow-hidden transition-all ease-in-out duration-300 ${
          index === openedIndex ? "h-auto mt-5" : "h-0 "
        }`}
      >
        {children}
      </div>
      <div
        className={`absolute right-6 top-6 w-10 h-10 rounded-full text-center middle
        ${index === openedIndex ? "bg-sky-500" : "bg-sky-900"}`}
      >
        <span className="inline-block pt-1 text-2xl font-semibold">
          {index === openedIndex ? "-" : "+"}
        </span>
      </div>
    </Col>
  </Row>
);

export const Faq = ({ currentUser }) => {
  const [openedIndex, setOpenedIndex] = useState(1);

  return (
    <InnerPageWrapper>
      <Header currentUser={currentUser} />

      <Wrapper>
        <Container className="text-white text-center mt-8">
          <InnerPageHead
            title={FAQContent.title}
            description={FAQContent.description}
          />

          <div className="my-12">
            <FaqQuestion
              index="0"
              openedIndex={openedIndex}
              changeOpened={(i) => setOpenedIndex(i)}
              title="What are Play-to-Earn Games?"
            >
              <p>
                An NFT game combines conventional gaming designs with
                unconventional game mechanisms to let users have more control
                over in-game assets like virtual mines, characters, items and
                much more. This is made possible by launching games on
                blockchains and anchoring them with digital asset-powered
                economies. These digital assets are often NFTs so that they are
                distinguishable and tamper-proof. The adoption of NFT token
                standards also allows developers to preserve the rarity and
                uniqueness of some of these in-game items.
              </p>
              <p className="mt-3">
                With this system in place, the players can claim ownership of
                game assets through 3 main strategies. They can mint new
                characters, purchase digital items on native or third-party
                marketplaces, or unlock and earn new items. Whichever way you
                choose to access these game assets, you have exclusive ownership
                rights over them. In essence, you can distribute or sell them
                and pocket all the money made from such trades. This is why this
                gaming model is called play-to-earn.
              </p>
            </FaqQuestion>

            <FaqQuestion
              index="1"
              openedIndex={openedIndex}
              changeOpened={(i) => setOpenedIndex(i)}
              title="How can I Play?"
            >
              <p>
                You need{" "}
                <a href="https://wallet.near.org/" target="_blank">
                  NEAR Wallet
                </a>{" "}
                and some NEAR tokens balance to play the game (used for
                blockchain transaction commissions, storage and in-game
                purchases). When you Login in MineLand using your NEAR Wallet,
                first of all you will need a Mine.
              </p>
              <h3 className="mt-3 font-semibold">Mines.</h3>
              <p>
                You can mint one Small Mine for free (0.01 NEAR that will be
                paid for storage) or buy larger mines to get more stones each
                day.
              </p>
              <p>
                Mines give you ability to catch (mint) stones each 24 hours:
              </p>
              <ul>
                <li>&minus; Small Mine: 1 stone/day.</li>
                <li>
                  &minus; Medium Mine: 4 stones/day.
                  <span className="text-sky-200 ml-1">
                    More chances (up to +20% than Small Mine) to get better Card
                    Rarity.
                  </span>
                </li>
                <li>
                  &minus; Large Mine: 8 stones/day.
                  <span className="text-sky-300 ml-1">
                    More chances (up to +50% than Small Mine) to get better Card
                    Rarity.
                  </span>
                </li>
              </ul>

              <h3 className="mt-3 font-semibold">Stones.</h3>
              <p>
                Stones is your main in-game items that will lead to achieve your
                goals. Each stone has its own characteristics of hardness,
                density and durability that affect its price. We have 4 types of
                Stone Card Rarity: Common, UnCommon, Rare and Legendary.
              </p>
              <p>
                Additionally: bigger Mine you use for minting - more chances you
                have to get rare cards.
              </p>

              <p className="mt-3">
                Main actions that you can perform with stones:
              </p>
              <ul>
                <li>
                  &minus; Create a collection - exchanges your stones for a real
                  monster with extra features.
                </li>
                <li>&minus; Sell in the market.</li>
                <li>
                  &minus; Kill to get MNL tokens (used for DAO and staking
                  rewards).
                </li>
                <li>&minus; Send to another user.</li>
              </ul>
            </FaqQuestion>

            <FaqQuestion
              index="2"
              openedIndex={openedIndex}
              changeOpened={(i) => setOpenedIndex(i)}
              title="How to buy Near (NEAR)?"
            >
              <p>
                <p>1. Register account in crypto exchanges.</p>
                <p className="pl-4">
                  The easiest way to buy Near is from a cryptocurrency exchange.
                  You can use one of the exchanges:
                  <a
                    href="https://www.binance.com/"
                    target="_blank"
                    className="ml-1"
                  >
                    Binance
                  </a>
                  ,
                  <a
                    href="https://www.huobi.com/"
                    target="_blank"
                    className="ml-1"
                  >
                    Huobi
                  </a>
                  ,
                  <a
                    href="https://www.kucoin.com/"
                    target="_blank"
                    className="ml-1"
                  >
                    Kukoin
                  </a>
                </p>

                <p>2. Create an account.</p>
                <p className="pl-4">
                  To create an account on an exchange you will need to verify
                  your email address and identity. Have some photo ID and your
                  phone ready.
                </p>
                <p>3. Make a deposit.</p>
                <p className="pl-4">
                  Once verified, you can deposit $ using the payment method that
                  best suits you – payments are widely accepted.
                </p>
                <p>4. Buy Near token.</p>
                <p className="pl-4">
                  You can now exchange your funds for Near. On easier-to-use
                  exchanges, this is as easy as entering the amount you want to
                  purchase and clicking buy. If you like you can now withdraw
                  your Near to your personal wallet.
                </p>
                <p>
                  5. Create{" "}
                  <a href="https://wallet.near.org/" target="_blank">
                    NEAR wallet
                  </a>
                  .
                </p>
                <p>6. Withdraw your NEAR tokens to your NEAR Wallet address.</p>
              </p>
            </FaqQuestion>
          </div>
        </Container>
      </Wrapper>

      <Footer />
    </InnerPageWrapper>
  );
};
