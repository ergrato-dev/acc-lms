import { Avatar, Button } from '@/components/ui';
import { useAuth } from '@/hooks/useAuth';
import { cn } from '@/utils';
import { ROUTES } from '@/utils/constants';
import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Link, useLocation, useNavigate } from 'react-router-dom';

export interface HeaderProps {
  className?: string;
}

/**
 * Main header/navigation component
 */
function Header({ className }: HeaderProps) {
  const { t, i18n } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();
  const { user, isAuthenticated, logout } = useAuth();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isUserMenuOpen, setIsUserMenuOpen] = useState(false);

  const toggleMobileMenu = useCallback(() => {
    setIsMobileMenuOpen((prev) => !prev);
  }, []);

  const handleLogout = useCallback(async () => {
    await logout();
    navigate(ROUTES.HOME);
  }, [logout, navigate]);

  const changeLanguage = useCallback(
    (lang: string) => {
      i18n.changeLanguage(lang);
    },
    [i18n]
  );

  const navLinks = [
    { href: ROUTES.COURSES.LIST, label: t('common:nav.courses') },
    { href: ROUTES.ABOUT, label: t('common:nav.about') },
    { href: ROUTES.CONTACT, label: t('common:nav.contact') },
  ];

  const isActiveLink = (href: string) => location.pathname === href;

  return (
    <header
      className={cn(
        'bg-background/95 supports-[backdrop-filter]:bg-background/60 sticky top-0 z-40 w-full border-b backdrop-blur',
        className
      )}
    >
      <div className="container mx-auto flex h-16 items-center justify-between px-4">
        {/* Logo */}
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
          <span className="hidden sm:inline">ACC LMS</span>
        </Link>

        {/* Desktop Navigation */}
        <nav className="hidden items-center gap-6 md:flex">
          {navLinks.map((link) => (
            <Link
              key={link.href}
              to={link.href}
              className={cn(
                'hover:text-primary text-sm font-medium transition-colors',
                isActiveLink(link.href)
                  ? 'text-primary'
                  : 'text-muted-foreground'
              )}
            >
              {link.label}
            </Link>
          ))}
        </nav>

        {/* Right side: Language, Auth, Mobile menu */}
        <div className="flex items-center gap-4">
          {/* Language Selector */}
          <div className="hidden items-center gap-1 sm:flex">
            {['es', 'en', 'pt'].map((lang) => (
              <button
                key={lang}
                onClick={() => changeLanguage(lang)}
                className={cn(
                  'rounded px-2 py-1 text-xs',
                  i18n.language === lang
                    ? 'bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:text-foreground'
                )}
              >
                {lang.toUpperCase()}
              </button>
            ))}
          </div>

          {/* Auth buttons / User menu */}
          {isAuthenticated && user ? (
            <div className="relative">
              <button
                onClick={() => setIsUserMenuOpen(!isUserMenuOpen)}
                className="flex items-center gap-2"
              >
                <Avatar
                  src={user.avatarUrl}
                  name={`${user.firstName} ${user.lastName}`}
                  size="sm"
                />
                <span className="hidden text-sm font-medium md:inline">
                  {user.firstName}
                </span>
              </button>

              {/* User dropdown */}
              {isUserMenuOpen && (
                <div className="bg-background absolute top-full right-0 mt-2 w-48 rounded-md border p-1 shadow-lg">
                  <div className="border-b px-3 py-2">
                    <p className="text-sm font-medium">
                      {user.firstName} {user.lastName}
                    </p>
                    <p className="text-muted-foreground text-xs">
                      {user.email}
                    </p>
                  </div>
                  <Link
                    to={ROUTES.DASHBOARD.ROOT}
                    className="hover:bg-accent block rounded px-3 py-2 text-sm"
                    onClick={() => setIsUserMenuOpen(false)}
                  >
                    {t('common:nav.dashboard')}
                  </Link>
                  <Link
                    to={ROUTES.DASHBOARD.MY_COURSES}
                    className="hover:bg-accent block rounded px-3 py-2 text-sm"
                    onClick={() => setIsUserMenuOpen(false)}
                  >
                    {t('common:nav.myCourses')}
                  </Link>
                  <Link
                    to={ROUTES.DASHBOARD.SETTINGS}
                    className="hover:bg-accent block rounded px-3 py-2 text-sm"
                    onClick={() => setIsUserMenuOpen(false)}
                  >
                    {t('common:nav.settings')}
                  </Link>
                  {user.role === 'instructor' && (
                    <Link
                      to={ROUTES.INSTRUCTOR.ROOT}
                      className="hover:bg-accent block rounded px-3 py-2 text-sm"
                      onClick={() => setIsUserMenuOpen(false)}
                    >
                      {t('common:nav.instructorDashboard')}
                    </Link>
                  )}
                  {user.role === 'admin' && (
                    <Link
                      to={ROUTES.ADMIN.ROOT}
                      className="hover:bg-accent block rounded px-3 py-2 text-sm"
                      onClick={() => setIsUserMenuOpen(false)}
                    >
                      {t('common:nav.adminPanel')}
                    </Link>
                  )}
                  <hr className="my-1" />
                  <button
                    onClick={() => {
                      setIsUserMenuOpen(false);
                      handleLogout();
                    }}
                    className="text-destructive hover:bg-accent w-full rounded px-3 py-2 text-left text-sm"
                  >
                    {t('common:actions.logout')}
                  </button>
                </div>
              )}
            </div>
          ) : (
            <div className="hidden items-center gap-2 sm:flex">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => navigate(ROUTES.AUTH.LOGIN)}
              >
                {t('common:actions.login')}
              </Button>
              <Button
                variant="primary"
                size="sm"
                onClick={() => navigate(ROUTES.AUTH.REGISTER)}
              >
                {t('common:actions.signUp')}
              </Button>
            </div>
          )}

          {/* Mobile menu button */}
          <button
            onClick={toggleMobileMenu}
            className="hover:bg-accent rounded-md p-2 md:hidden"
            aria-label="Toggle menu"
          >
            {isMobileMenuOpen ? (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            ) : (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <line x1="4" y1="12" x2="20" y2="12" />
                <line x1="4" y1="6" x2="20" y2="6" />
                <line x1="4" y1="18" x2="20" y2="18" />
              </svg>
            )}
          </button>
        </div>
      </div>

      {/* Mobile Menu */}
      {isMobileMenuOpen && (
        <div className="bg-background border-t md:hidden">
          <nav className="container mx-auto px-4 py-4">
            {navLinks.map((link) => (
              <Link
                key={link.href}
                to={link.href}
                className={cn(
                  'block py-2 text-sm font-medium',
                  isActiveLink(link.href)
                    ? 'text-primary'
                    : 'text-muted-foreground'
                )}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                {link.label}
              </Link>
            ))}
            <hr className="my-2" />
            {!isAuthenticated && (
              <div className="mt-4 flex flex-col gap-2">
                <Button
                  variant="outline"
                  fullWidth
                  onClick={() => {
                    setIsMobileMenuOpen(false);
                    navigate(ROUTES.AUTH.LOGIN);
                  }}
                >
                  {t('common:actions.login')}
                </Button>
                <Button
                  variant="primary"
                  fullWidth
                  onClick={() => {
                    setIsMobileMenuOpen(false);
                    navigate(ROUTES.AUTH.REGISTER);
                  }}
                >
                  {t('common:actions.signUp')}
                </Button>
              </div>
            )}
            {/* Language selector mobile */}
            <div className="mt-4 flex items-center gap-2">
              <span className="text-muted-foreground text-sm">
                {t('common:language')}:
              </span>
              {['es', 'en', 'pt'].map((lang) => (
                <button
                  key={lang}
                  onClick={() => changeLanguage(lang)}
                  className={cn(
                    'rounded px-2 py-1 text-xs',
                    i18n.language === lang
                      ? 'bg-primary text-primary-foreground'
                      : 'text-muted-foreground hover:text-foreground'
                  )}
                >
                  {lang.toUpperCase()}
                </button>
              ))}
            </div>
          </nav>
        </div>
      )}
    </header>
  );
}

export default Header;
