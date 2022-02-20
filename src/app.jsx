import React, { useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { initContract } from "./near/api";
import {
  Collections,
  ContactUs,
  Faq,
  Mines,
  Market,
  Stones,
} from "./pages";
import { Terms } from "./pages/Terms";
import { Privacy } from "./pages/Privacy";
import { Sidebar } from "./components/sidebar/Sidebar";
import { Token } from './pages/Token';
import { Landing } from './pages/landing/Landing';

export const App = () => {
  const [currentUser, setCurrentUser] = React.useState(false);
  const [contract, setContract] = React.useState(false);
  const [ftContract, setFtContract] = React.useState(false);
  const [isReady, setIsReady] = React.useState(false);
  const [sellList, setSellList] = React.useState({
    mines: [],
    stones: [],
  });
  const [sidebarIsOpen, setSidebarIsOpen] = useState(false);

  React.useEffect(() => {
    window.nearInitPromise = initContract()
      .then(async () => {
        setContract(window.contract);
        setFtContract(window.ftContract);

        if (window.walletConnection.isSignedIn()) {
          const accountId = window.walletConnection?.getAccountId();
          let tokenBalance = await window.ftContract.ft_balance_of({
            account_id: accountId
          });

          setCurrentUser({
            accountId: accountId,
            tokenBalance: tokenBalance,
          });
        } else {
          console.log("Not logged in");
        }

        setIsReady(true);
      })
      .catch(console.error);
  }, []);

  React.useEffect(() => {
    setSidebarIsOpen(true);
  }, [sellList]);

  return (
    <BrowserRouter>
      {isReady && (
        <>
          <Routes>
            <Route
              exact
              path="/"
              element={
                <Landing currentUser={currentUser} contract={contract} />
              }
            />
            <Route
              exact
              path="/mines"
              element={
                <Mines
                  currentUser={currentUser}
                  contract={contract}
                  sellList={sellList}
                  setSellList={setSellList}
                />
              }
            />
            <Route
              exact
              path="/stones"
              element={
                <Stones
                  currentUser={currentUser}
                  contract={contract}
                  sellList={sellList}
                  setSellList={setSellList}
                />
              }
            />
            <Route
              exact
              path="/collections"
              element={
                <Collections currentUser={currentUser} contract={contract} />
              }
            />
            <Route
              exact
              path="/market"
              element={<Market currentUser={currentUser} contract={contract} />}
            />
            <Route
              exact
              path="/token"
              element={<Token currentUser={currentUser} />}
            />
            <Route
              exact
              path="/faq"
              element={<Faq currentUser={currentUser} contract={contract} />}
            />
            <Route
              exact
              path="/contact-us"
              element={<ContactUs currentUser={currentUser} />}
            />
            <Route
              exact
              path="/terms-conditions"
              element={<Terms currentUser={currentUser} />}
            />
            <Route
              exact
              path="/privacy-policy"
              element={<Privacy currentUser={currentUser} />}
            />
          </Routes>

          <Sidebar
            currentUser={currentUser}
            contract={contract}
            sellList={sellList}
            setSellList={setSellList}
            isOpen={sidebarIsOpen}
            setIsOpen={setSidebarIsOpen}
          />
        </>
      )}
    </BrowserRouter>
  );
};
