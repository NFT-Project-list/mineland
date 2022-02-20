import { useEffect, useRef, useState } from "react";
import { convertFromNanoSeconds, getMedia } from "near/api";
import { formatId } from "utils/index";
import { Button } from "components/basic/Button";
import { Popup } from "components/Popup";

export default function MintStonePopup({
  mintPopupVisible,
  setMintPopupVisible,
  userMines,
  handleMint,
}) {
  const funRef = useRef(null);
  const [currentDate, setCurrentDate] = useState(Date.now());
  const availabilityMap = {
    Small: "1 stone / day",
    Medium: "4 stones / day",
    Large: "8 stones / day",
  };

  useEffect(() => {
    funRef.current = setInterval(() => {
      setCurrentDate(Date.now());
    }, 1000);
    return () => {
      clearInterval(funRef.current);
    };
  }, []);

  useEffect(() => {
    // used for update each second
  }, [currentDate]);

  const secondsToString = (countSec) => {
    let hours = Math.floor(((countSec % 31536000) % 86400) / 3600);
    let minutes = Math.floor((((countSec % 31536000) % 86400) % 3600) / 60);
    let seconds = Math.floor((((countSec % 31536000) % 86400) % 3600) % 60);
    if (minutes < 10) {
      minutes = `0${minutes}`;
    }
    if (seconds < 10) {
      seconds = `0${seconds}`;
    }
    return hours + " hours " + minutes + " min " + seconds + " sec.";
  };

  const timeDiff = (timeInMs) => {
    const timeNow = new Date().getTime();
    const oneDay = 24 * 60 * 60 * 1000;
    const lastClaimTime = convertFromNanoSeconds(timeInMs);
    const diff = timeNow - lastClaimTime;
    return (oneDay - diff) / 1000;
  };

  return (
    <Popup
      title="Mint Stones"
      popupVisible={mintPopupVisible}
      setPopupVisible={setMintPopupVisible}
    >
      <div className="mt-2 text-left">
        {userMines.map((mine) => (
          <div className="flex gap-4 mb-3" key={mine.token_id}>
            <div>
              <img src={getMedia(mine.media)} alt="mine" width="40" />
            </div>
            <div className="basis-1/3 pt-4 font-semibold">
              {mine.mine_type} Mine #{formatId(mine.token_id)}
            </div>
            <div className="basis-1/4 pt-4">
              {availabilityMap[mine.mine_type]}
            </div>
            <div className="grow text-right">
              {timeDiff(mine.last_stone_claim) < 0 ? (
                <div className="pt-1">
                  <Button
                    title="Mint Stones"
                    size="sm"
                    secondary
                    onClick={() => handleMint(mine.token_id, mine.mine_type)}
                  />
                </div>
              ) : (
                <p className="text-red-300 text-center pl-7 text-base pt-2 leading-4 font-[Exo]">
                  <small>Next mint:</small>
                  <br />
                  <small>
                    {secondsToString(timeDiff(mine.last_stone_claim))}
                  </small>
                </p>
              )}
            </div>
          </div>
        ))}
      </div>
    </Popup>
  );
}
