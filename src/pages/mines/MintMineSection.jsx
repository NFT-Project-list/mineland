import { convertFromYocto, convertToYocto, defaultGas } from "near/api";
import { Row, Col } from "assets/styles/common.style";
import { Button } from "components/basic/Button";
import { Card } from "../../components/card/Card";

export const MintMineSection = ({
  currentUser,
  contract,
  minesCount,
  allMines,
}) => {
  const MintCard = ({ type, handleMint }) => (
    <Col>
      <Card noFlip nft={allMines[type]} />
      <div className="mt-7">
        <Button title={`Mint ${type} Mine`} onClick={handleMint} />
        <div className="mt-3 font-semibold">
          {convertFromYocto(allMines[type].price)} NEAR
        </div>
      </div>
    </Col>
  );

  const handleMint = async (depositAmount) => {
    const deposit = convertToYocto(depositAmount);
    await contract.mint_mine_nft(
      {
        account_id: currentUser.accountId,
      },
      defaultGas,
      deposit
    );
  };

  return (
    <Row className="justify-center gap-12">
      {allMines && (
        <>
          {!minesCount && (
            <MintCard type="Small" handleMint={() => handleMint(0.01)} />
          )}
          <MintCard type="Medium" handleMint={() => handleMint(5)} />
          <MintCard type="Large" handleMint={() => handleMint(9)} />
        </>
      )}
    </Row>
  );
};
