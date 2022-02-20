import React from "react";
import {
  Container,
  InnerPageWrapper,
  Wrapper,
} from "assets/styles/common.style";
import { Header } from "components/Header";
import { Footer } from "components/Footer";
import { InnerPageHead } from "components/InnerPageHead";
import { PrivacyContent } from "utils/content";

export const Privacy = ({ currentUser }) => (
  <InnerPageWrapper>
    <Header currentUser={currentUser} />

    <Wrapper>
      <Container className="text-white text-center mt-8">
        <InnerPageHead
          title={PrivacyContent.title}
          description={PrivacyContent.description}
        />
        ...
      </Container>
    </Wrapper>

    <Footer />
  </InnerPageWrapper>
);
