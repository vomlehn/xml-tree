# xml-tree
FIXME: Needs work

XML parsing components. The names of most of the files use the convention
<operation>_<target>.

<operation> may be
     parse - actions are performed during the parse
     walk - an XML tree is created first, then actions are pe4formed during a tree walk.

<target> may be
     doc - the entire document is parsed while inherited actions are performed
     echo - the action is to output the original XML input
     tree - the action is to build an XML tree and return it
     schema - Output the Rust code for a schema
