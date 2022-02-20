import { Col, Container, Row, Wrapper } from "assets/styles/common.style";
import { Header } from "components/Header";
import { login } from "near/api";
import { Circle } from "assets/styles/home.style";
import mint_mine from "assets/images/mint_mine.png";
import { ChevronDoubleRightIcon } from "@heroicons/react/solid";
import { useNavigate } from "react-router-dom";
import { Button } from "components/basic/Button";
import { HomeContent } from "utils/content";
import { HowToPlaySection } from "./HowToPlaySection";
import { RoadmapSection } from "./RoadmapSection";
import { Section } from "./Section";
import { Footer } from "components/Footer";
import { Timeline } from "assets/styles/home.style";

const CircleSection = ({ number, title, desc }) => (
  <Col className="mx-10 text-center items-center leading-normal">
    <Circle>{number}</Circle>
    <h2 className="font-semibold text-2xl mt-5 mb-3">{title}</h2>
    <p>{desc}</p>
  </Col>
);

export const Landing = ({ currentUser }) => {
  let navigate = useNavigate();

  const handleClick = () => (currentUser ? navigate("/stones") : login());

  return (
    <div className="mineing-bg">
      <Wrapper className="top-block">
        <Header currentUser={currentUser} />

        <Container className="pt-36 mb-24">
          <h1 className="stone-font text-7xl leading-tight w-1/2 title-shadow">
            {HomeContent.play_to_earn.title}
          </h1>
          <h3 className="mt-6 mb-10 text-xl leading-normal w-1/2">
            {HomeContent.play_to_earn.desc}
          </h3>
          <Button animated size="lg" title="Play Game" onClick={handleClick} />
        </Container>

        <Container className="py-10">
          <Section
            title={HomeContent.how_to_play.title}
            description={HomeContent.how_to_play.desc}
          >
            <Row className="mt-16 justify-around">
              <CircleSection
                number="1"
                title={HomeContent.login_using_near.title}
                desc={HomeContent.login_using_near.desc}
              />
              <ChevronDoubleRightIcon width="150" />
              <CircleSection
                number="2"
                title={HomeContent.mint_or_buy.title}
                desc={HomeContent.mint_or_buy.desc}
              />
              <ChevronDoubleRightIcon width="150" />
              <CircleSection
                number="3"
                title={HomeContent.catch_stonez.title}
                desc={HomeContent.catch_stonez.desc}
              />
            </Row>
          </Section>
        </Container>

        <Container>
          <HowToPlaySection
            title={HomeContent.how_to_play.title}
            desc={HomeContent.how_to_play.desc}
            img={mint_mine}
          />
          <HowToPlaySection
            title={HomeContent.how_to_play.title}
            desc={HomeContent.how_to_play.desc}
            img={mint_mine}
            reverse
          />
          <HowToPlaySection
            title={HomeContent.how_to_play.title}
            desc={HomeContent.how_to_play.desc}
            img={mint_mine}
          />
        </Container>
        <Container className="py-20">
          <Section
            title={HomeContent.roadmap.title}
            description={HomeContent.roadmap.desc}
          >
            <Col>
              <Timeline>
                {HomeContent.roadmap.sections.map((section, index) => (
                  <RoadmapSection
                    key={index}
                    date={section.date}
                    title={section.title}
                    desc={section.desc}
                    type={section.type}
                  />
                ))}
              </Timeline>
            </Col>
          </Section>
        </Container>

        <Footer />
      </Wrapper>
    </div>
  );
};
