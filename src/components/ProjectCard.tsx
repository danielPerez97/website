import React, { FC } from 'react';
import styled from 'styled-components';

const CardContainer = styled.div`
  border-width: 3px;
  margin: 0px;
  height: 10px;
  width: 563px;
`;

const ProjectName = styled.div`
  font-family: Georgia, serif;
  font-size: xxx-large;
  margin-top: 25px;
  margin-left: 0px;
`;

const ProjectDescription = styled.div`
  font-family: Georgia, serif;
  font-size: large;
  margin-top: 25px;
  margin-left: 0px;
`;

interface ProjectCardProps {
    projectName: string,
    projectDescription: string,
}

const ProjectCard: FC<ProjectCardProps> = ({
  projectName,
  projectDescription,
}) => {
  return (
    <CardContainer>
      <ProjectName>
        {projectName}
      </ProjectName>
      <ProjectDescription>
        {projectDescription}
      </ProjectDescription>
    </CardContainer>
  );
}

export default ProjectCard;
