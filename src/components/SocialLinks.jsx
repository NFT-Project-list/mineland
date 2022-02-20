import React from "react";
import twitterIcon from "assets/images/twitter.png";
import telegramIcon from "assets/images/telegram.png";
import discordIcon from "assets/images/discord.png";
import { Link } from "assets/styles/common.style";

export default function SocialLinks() {
  return (
    <div className="flex flex-row justify-end">
      <Link to="/" className="hover:opacity-75">
        <img src={twitterIcon} alt="tw" className="w-7" />
      </Link>
      <Link to="/" className="hover:opacity-75">
        <img src={discordIcon} alt="tw" className="w-7 ml-3" />
      </Link>
      <Link to="/" className="hover:opacity-75">
        <img src={telegramIcon} alt="tw" className="w-7 ml-3" />
      </Link>
    </div>
  );
}
