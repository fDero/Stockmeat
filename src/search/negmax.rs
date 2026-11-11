/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::scores::Score;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ApproxScore {
    Exact(Score),
    LessThan(Score),
    GreaterThan(Score),
}
