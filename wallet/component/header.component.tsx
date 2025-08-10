import {
  ConnectWallet,
  SmartContract,
  useActiveClaimConditionForWallet,
  useAddress,
  useClaimConditions,
  useClaimedNFTSupply,
  useContract,
  useUnclaimedNFTSupply,
  Web3Button
} from '@thirdweb-dev/react';
import { BaseContract } from 'ethers/lib/ethers';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { assets, SvgIcons } from '../assets';
import { HeaderButton } from '../button';
import { StyledModalWindow } from '../styled-modal-window';
import { headerTexts, menuLinks } from './header.data';
import * as Styled from './header.styles';

const contractAddress = '0x0000000000000000000000000000000000000000';

export const mint = async (contract: SmartContract<BaseContract>) => {
  contract.erc721.claim(1);
};

export const Header: React.FC = () => {
  const navigate = useNavigate();
  const [isOpen, setOpen] = useState(false);
  const [isClosing, setClosing] = useState(false);

  const { contract, isLoading } = useContract(contractAddress);

  const address = useAddress();
  const claimConditions = useClaimConditions(contract);
  const activeClaimCondition = useActiveClaimConditionForWallet(contract, address || '');

  const unclaimedSupply = useUnclaimedNFTSupply(contract);

  const claimedSupply = useClaimedNFTSupply(contract);

  const handleClosing = async () => {
    await setClosing(true);
  };

  const handleClose = () => {
    setOpen(false);
    setClosing(false);
  };

  const handleRedirect = (link: string) => {
    navigate(`..${link}`);
  };

  return (
    <Styled.HeaderWrapper>
      <Styled.LeftBlock>
        <Styled.BurgerWrapper onClick={() => setOpen(true)}>
          <SvgIcons.Burger />
        </Styled.BurgerWrapper>

        <Styled.LogoBlock>
          <Styled.LogoImgWrapper src={assets.images.headerLogo} />
          <Styled.LogoTextWrapper src={assets.images.headerTextLogo} />
        </Styled.LogoBlock>
      </Styled.LeftBlock>
      <Styled.NavigateBlock>
        <HeaderButton text={headerTexts}>
          <Web3Button
            style={{
              padding: '0',
              background: 'none',
              fontStyle: 'normal',
              fontWeight: '400',
              fontSize: '27px',
              textAlign: 'center',
              color: '#ffffff',
              border: 'red 2px solid'
            }}
            className="connectwalletbutton"
            contractAddress="0xe7d6244B303eE3c6150aD05980EEAf8394f7a0aC"
            action={mint}>
            <ConnectWallet
              theme="dark"
              btnTitle="Connect Wallet"
              dropdownPosition={{ side: 'right', align: 'start' }}
            />
          </Web3Button>
        </HeaderButton>
      </Styled.NavigateBlock>
      {isOpen && (
        <StyledModalWindow
          top="0px"
          left="0px"
          isClosingProcess={isClosing}
          handleClose={handleClose}>
          <Styled.MenuWrapper>
            <Styled.MenuIconsWrapper>
              <SvgIcons.Close onClick={handleClosing} />
            </Styled.MenuIconsWrapper>
            <Styled.MenuLinksWrapper>
              {menuLinks.map((menuLink) => (
                <Styled.MenuLink onClick={() => handleRedirect(menuLink.link)}>
                  {menuLink.title}
                </Styled.MenuLink>
              ))}
            </Styled.MenuLinksWrapper>
            <Styled.MenuMediaWrapper>
              <SvgIcons.Facebook />
              <SvgIcons.Discord />
              <SvgIcons.Twitter />
              <SvgIcons.YouTube />
              <SvgIcons.Instagram />
            </Styled.MenuMediaWrapper>
          </Styled.MenuWrapper>
        </StyledModalWindow>
      )}
    </Styled.HeaderWrapper>
  );
};
