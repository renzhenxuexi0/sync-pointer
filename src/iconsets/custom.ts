import type { IconAliases, IconProps, IconSet } from 'vuetify';
import MdiChevronUp from '~icons/mdi/chevron-up';
import MdiCheck from '~icons/mdi/check';
import MdiCancel from '~icons/mdi/cancel';
import MdiClose from '~icons/mdi/close';
import MdiDelete from '~icons/mdi/delete';
import MdiCloseCircle from '~icons/mdi/close-circle';
import MdiCheckCircle from '~icons/mdi/check-circle';
import MdiInformation from '~icons/mdi/information';
import MdiAlert from '~icons/mdi/alert';
import MdiAlertCircle from '~icons/mdi/alert-circle';
import MdiChevronLeft from '~icons/mdi/chevron-left';
import MdiChevronRight from '~icons/mdi/chevron-right';
import MdiCheckboxMarked from '~icons/mdi/checkbox-marked';
import MdiCheckboxBlankOutline from '~icons/mdi/checkbox-blank-outline';
import MdiCheckboxIntermediate from '~icons/mdi/checkbox-intermediate';
import MdiCircle from '~icons/mdi/circle';
import MdiArrowUp from '~icons/mdi/arrow-up';
import MdiChevronDown from '~icons/mdi/chevron-down';
import MdiMenu from '~icons/mdi/menu';
import MdiMenuDown from '~icons/mdi/menu-down';
import MdiRadioboxMarked from '~icons/mdi/radiobox-marked';
import MdiRadioboxBlank from '~icons/mdi/radiobox-blank';
import MdiPencil from '~icons/mdi/pencil';
import MdiStarOutline from '~icons/mdi/star-outline';
import MdiStar from '~icons/mdi/star';
import MdiStarHalfFull from '~icons/mdi/star-half-full';
import MdiLoading from '~icons/mdi/loading';
import MdiPageFirst from '~icons/mdi/page-first';
import MdiPageLast from '~icons/mdi/page-last';
import MdiUnfoldMoreHorizontal from '~icons/mdi/unfold-more-horizontal';
import MdiFile from '~icons/mdi/file';
import MdiPlus from '~icons/mdi/plus';
import MdiMinus from '~icons/mdi/minus';
import MdiSortAscending from '~icons/mdi/sort-ascending';
import MdiSortDescending from '~icons/mdi/sort-descending';
import MdiCalendar from '~icons/mdi/calendar';

const aliases: IconAliases = {
    collapse: MdiChevronUp,
    complete: MdiCheck,
    cancel: MdiCancel,
    close: MdiClose,
    delete: MdiDelete,
    clear: MdiCloseCircle,
    success: MdiCheckCircle,
    info: MdiInformation,
    warning: MdiAlert,
    error: MdiAlertCircle,
    prev: MdiChevronLeft,
    next: MdiChevronRight,
    checkboxOn: MdiCheckboxMarked,
    checkboxOff: MdiCheckboxBlankOutline,
    checkboxIndeterminate: MdiCheckboxIntermediate,
    delimiter: MdiCircle,
    sort: MdiArrowUp,
    expand: MdiChevronDown,
    menu: MdiMenu,
    subgroup: MdiMenuDown,
    dropdown: MdiMenuDown,
    radioOn: MdiRadioboxMarked,
    radioOff: MdiRadioboxBlank,
    edit: MdiPencil,
    ratingEmpty: MdiStarOutline,
    ratingFull: MdiStar,
    ratingHalf: MdiStarHalfFull,
    loading: MdiLoading,
    first: MdiPageFirst,
    last: MdiPageLast,
    unfold: MdiUnfoldMoreHorizontal,
    file: MdiFile,
    plus: MdiPlus,
    minus: MdiMinus,
    sortAsc: MdiSortAscending,
    sortDesc: MdiSortDescending,
    calendar: MdiCalendar,
};

const custom: IconSet = {
    component: async (props: IconProps) => {
        const { icon } = props;
        return icon;
    },
};

export { aliases, custom };
