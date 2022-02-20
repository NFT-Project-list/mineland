import React from "react";
import { Container, InnerPageWrapper, Wrapper } from 'assets/styles/common.style';
import { Header } from "components/Header";
import { Footer } from 'components/Footer';
import { ContactUsContent } from 'utils/content';
import { InnerPageHead } from 'components/InnerPageHead';

export const ContactUs = ({ currentUser }) => (
  <>
    <InnerPageWrapper>
      <Header currentUser={currentUser} />
      <Wrapper>
        <Container className="text-white text-center mt-8">
          <InnerPageHead title={ContactUsContent.title} description={ContactUsContent.description} />
          ...
        </Container>
      </Wrapper>
      <Footer />
    </InnerPageWrapper>
  </>
);
