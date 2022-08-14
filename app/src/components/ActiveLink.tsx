import { withRouter, NextRouter } from 'next/router';
import React, { ReactElement } from 'react';
import Link from 'next/link';

type Props = {
  router: NextRouter;
  children: ReactElement;
  href: string;
  activeClassName: string;
};

const ActiveLink = ({ router, children, ...props }: Props) => {
  const child = children;

  let className: string = child.props.className;
  if (router.pathname == props.href) {
    className = className
      ? `${className} ${props.activeClassName}`
      : `${props.activeClassName}`;
  }

  return <Link {...props}>{React.cloneElement(child, { className })}</Link>;
};

export default withRouter(ActiveLink);
