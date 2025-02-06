import { useTranslations } from "next-intl";

export default function Home() {
  const t = useTranslations('home');
  return (
    <div className="w-full h-full">
      {t('app.title')}
    </div>
  );
}
