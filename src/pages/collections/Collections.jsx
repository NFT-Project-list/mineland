import React, { useEffect, useState } from "react";
import { Header } from "components/Header";
import { Col, Container, InnerPageWrapper, Link, Row, Wrapper } from "assets/styles/common.style";
import { InnerPageHead } from 'components/InnerPageHead';
import { Loader } from 'components/basic/Loader';
import { CollectionContent } from 'utils/content';
import { Footer } from 'components/Footer';
import { getMedia } from 'near/api';
import { Button } from '../../components/basic/Button';

export const Collections = ({ currentUser, contract }) => {
  const [allCollections, setAllCollections] = useState([]);
  const [userCollectionCount, setUserCollectionCount] = useState({});
  const [isReady, setIsReady] = React.useState(false);

  useEffect(() => {
    const allCollectionsPromise = new Promise(async (resolve, reject) => {
      const collections = await contract.get_collections().catch(err => reject(err));
      const collection_list = Object.keys(collections).map(key => {
          return {
            id: key,
            ...collections[key],
          }
        }
      );
      resolve(collection_list);
    });

    const userCollectionsPromise = new Promise(async (resolve, reject) => {
      const userCollectionCounts = await contract.user_collection_counts({
        account_id: currentUser.accountId
      }).catch(err => reject(err));
      resolve(userCollectionCounts);
    });

    Promise.all([allCollectionsPromise, userCollectionsPromise]).then((result) => {
      setAllCollections(result[0]);
      setUserCollectionCount(result[1]);
      setIsReady(true);
    });
  }, []);

  return (
    <>
      <InnerPageWrapper>
        <Header currentUser={currentUser} />

        <Wrapper>
          <Container className="text-white text-center mt-8">
            <InnerPageHead title={CollectionContent.title} description={CollectionContent.description} />

            {isReady ? (
              <div className="flex flex-row flex-wrap text-left">
                {
                  allCollections.map(collection => (
                    <div className="basis-1/2 my-10 flex gap-7" key={collection.id}>
                      <div className="w-1/3 bg-[#0e0737]">
                        <img src={getMedia(collection.image)} alt={`collection #${collection.id}`}
                             className="bg-cover h-full w-full border-4 rounded-xl border-gray-500" />
                      </div>
                      <div className="w-2/3">
                        <p className="text-2xl mb-4 mt-4 font-semibold">
                          {collection.title}
                        </p>
                        <div className="font-semibold">
                          Your stones: {userCollectionCount[collection.id] ?? 0}
                        </div>
                        <div className="pr-16 mb-5 mt-1 text-sm font-normal">
                          Your collection of stones. Later you will be able to open and see all collection items and
                          your own NFTs.
                        </div>
                      </div>
                    </div>
                  ))
                }
              </div>
            ) : (
              <Loader />
            )}
          </Container>
        </Wrapper>

        <Footer />
      </InnerPageWrapper>
    </>
  )
};
