import styled from "styled-components";

export const Circle = styled.div.attrs({
  className: `
    flex
    items-center
    justify-center
    border-4
    border-teal-700
    rounded-full
    h-24
    w-24
    text-4xl
    stone-font
  `,
})``;

export const Timeline = styled.div.attrs({
  className: `
    relative
    pl-10
    before:content-['']
    before:absolute
    before:top-0
    before:left-0
    before:w-[2px]
    before:h-full
    before:bg-gray-500
  `,
})``;
