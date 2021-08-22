import React from 'react';
import PropTypes from 'prop-types';
import styled from 'styled-components';

const CardContainer = styled.div`
  padding: 10%;
  border-width: 3px;
  margin: 5px;
  height: 1000px;
  width: 563px;
`;

const ProjectName = styled.div`
  font-family: Georgia, serif;
  font-size: xxx-large;
  margin-top: 25px;
  margin-left: 25px;
`;

const ProjectDescription = styled.div`
  font-family: Georgia, serif;
  font-size: large;
  margin-top: 25px;
  margin-left: 25px;
`;

const ProjectCard = ({
  projectName,
  projectDescription,
}) => (
  <CardContainer>
    <ProjectName>
      {projectName}
    </ProjectName>
    <ProjectDescription>
      {projectDescription}
    </ProjectDescription>
  </CardContainer>
);

ProjectCard.propTypes = {
  projectName: PropTypes.string.isRequired,
  projectDescription: PropTypes.string.isRequired,
};

export default ProjectCard;
