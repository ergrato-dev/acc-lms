import { cn } from '@/utils';
import { ROUTES } from '@/utils/constants';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router-dom';

export interface FooterProps {
  className?: string;
}

/**
 * Main footer component
 */
function Footer({ className }: FooterProps) {
  const { t } = useTranslation();
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    platform: [
      { href: ROUTES.COURSES.LIST, label: t('common:nav.courses') },
      { href: ROUTES.ABOUT, label: t('common:nav.about') },
      { href: '/pricing', label: t('common:nav.pricing') },
      { href: '/blog', label: t('common:nav.blog') },
    ],
    support: [
      { href: ROUTES.HELP.ROOT, label: t('common:nav.helpCenter') },
      { href: ROUTES.CONTACT, label: t('common:nav.contact') },
      { href: '/faq', label: t('common:nav.faq') },
    ],
    legal: [
      { href: ROUTES.LEGAL.TERMS, label: t('common:nav.terms') },
      { href: ROUTES.LEGAL.PRIVACY, label: t('common:nav.privacy') },
      { href: ROUTES.LEGAL.COOKIES, label: t('common:nav.cookies') },
    ],
  };

  const socialLinks = [
    {
      name: 'Facebook',
      href: 'https://facebook.com',
      icon: (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z" />
        </svg>
      ),
    },
    {
      name: 'Twitter',
      href: 'https://twitter.com',
      icon: (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" />
        </svg>
      ),
    },
    {
      name: 'LinkedIn',
      href: 'https://linkedin.com',
      icon: (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
          <rect width="4" height="12" x="2" y="9" />
          <circle cx="4" cy="4" r="2" />
        </svg>
      ),
    },
    {
      name: 'YouTube',
      href: 'https://youtube.com',
      icon: (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z" />
          <polygon
            points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"
            fill="white"
          />
        </svg>
      ),
    },
  ];

  return (
    <footer className={cn('bg-muted/30 border-t', className)}>
      <div className="container mx-auto px-4 py-12">
        <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-5">
          {/* Brand section */}
          <div className="lg:col-span-2">
            <Link
              to={ROUTES.HOME}
              className="text-primary flex items-center gap-2 text-xl font-bold"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M22 10v6M2 10l10-5 10 5-10 5z" />
                <path d="M6 12v5c3 3 9 3 12 0v-5" />
              </svg>
              <span>ACC LMS</span>
            </Link>
            <p className="text-muted-foreground mt-4 max-w-xs text-sm">
              {t('common:footer.description')}
            </p>
            {/* Social links */}
            <div className="mt-6 flex gap-4">
              {socialLinks.map((social) => (
                <a
                  key={social.name}
                  href={social.href}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-muted-foreground hover:text-primary transition-colors"
                  aria-label={social.name}
                >
                  {social.icon}
                </a>
              ))}
            </div>
          </div>

          {/* Platform links */}
          <div>
            <h3 className="text-sm font-semibold">
              {t('common:footer.platform')}
            </h3>
            <ul className="mt-4 space-y-2">
              {footerLinks.platform.map((link) => (
                <li key={link.href}>
                  <Link
                    to={link.href}
                    className="text-muted-foreground hover:text-foreground text-sm transition-colors"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Support links */}
          <div>
            <h3 className="text-sm font-semibold">
              {t('common:footer.support')}
            </h3>
            <ul className="mt-4 space-y-2">
              {footerLinks.support.map((link) => (
                <li key={link.href}>
                  <Link
                    to={link.href}
                    className="text-muted-foreground hover:text-foreground text-sm transition-colors"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Legal links */}
          <div>
            <h3 className="text-sm font-semibold">
              {t('common:footer.legal')}
            </h3>
            <ul className="mt-4 space-y-2">
              {footerLinks.legal.map((link) => (
                <li key={link.href}>
                  <Link
                    to={link.href}
                    className="text-muted-foreground hover:text-foreground text-sm transition-colors"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Bottom section */}
        <div className="mt-12 border-t pt-6">
          <div className="flex flex-col items-center justify-between gap-4 sm:flex-row">
            <p className="text-muted-foreground text-sm">
              © {currentYear} ACC LMS. {t('common:footer.rights')}
            </p>
            <div className="text-muted-foreground flex items-center gap-4 text-sm">
              <span>{t('common:footer.madeWith')} ❤️</span>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}

export default Footer;
