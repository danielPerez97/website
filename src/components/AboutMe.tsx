import React from 'react';
import styled from 'styled-components';

const Banner = styled.h1`
  font-family: monospace;
`;

function AboutMe() {
  return (
    <Banner>
      My name is Daniel Perez. By trade I am an Android Developer well-versed in Kotlin,
      Dagger 2/Hilt, Retrofit, OkHttp, and Jetpack Compose. I was a Systems Engineer at Walmart
      working on Active Directory until I moved over to Govenda to become an Android engineer.
      I am known to dabble with Rust on occasion as well.
    </Banner>
  );
}

export default AboutMe;
