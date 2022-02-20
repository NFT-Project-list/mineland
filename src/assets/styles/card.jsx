import styled from "styled-components";
import { statusColorBorderMap } from "utils";

export const CardInner = styled.div.attrs(() => ({
  className: `relative h-full w-full text-center`,
}))`
  transition: transform 0.6s;
  transform-style: preserve-3d;
`;

export const FlipCard = styled.div`
  width: 220px;
  height: 328px;
  perspective: 600px;

  &.small {
    width: 140px;
    height: 198px;
    perspective: 300px;
  }

  &:hover ${CardInner} {
    transform: ${(props) => !props.noFlip && "rotateY(180deg)"};
  }
`;

export const CardFront = styled.div.attrs((props) => ({
  className: `
    absolute
    w-full
    h-full
    rounded-xl
    ${statusColorBorderMap(props.type)}
  `,
}))`
  -webkit-backface-visibility: hidden;
  backface-visibility: hidden;
`;

export const Rarity = styled.div.attrs((props) => ({
  className: `
    uppercase
    font-semibold
    text-gray-900
  `,
}))``;

export const CardBack = styled(CardFront)`
  transform: rotateY(180deg);
`;
