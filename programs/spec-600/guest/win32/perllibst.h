/*DO NOT EDIT
  this file is included from perllib.c to init static extensions */
#ifdef STATIC1
"Cwd",
"Data::Dumper",
"Devel::Peek",
"Digest::MD5",
"Digest::SHA",
  "File::Glob",
  "Hash::Util",
  "Hash::Util::FieldHash",
"HTML::Parser",
"IO",
  "List::Util",
"MIME::Base64",
"Opcode",
  "PerlIO::scalar",
  "PerlIO::via",
"Storable",
"Sys::Hostname",
  "Tie::Hash::NamedCapture",
"Time::HiRes",
  "arybase",
"attributes",
  "mro",
  "re",
#undef STATIC1
#endif
#ifdef STATIC2
EXTERN_C void boot_Cwd (pTHX_ CV* cv);
EXTERN_C void boot_Data__Dumper (pTHX_ CV* cv);
EXTERN_C void boot_Devel__Peek (pTHX_ CV* cv);
EXTERN_C void boot_Digest__MD5 (pTHX_ CV* cv);
EXTERN_C void boot_Digest__SHA (pTHX_ CV* cv);
EXTERN_C void boot_File__Glob (pTHX_ CV* cv);
EXTERN_C void boot_Hash__Util (pTHX_ CV* cv);
EXTERN_C void boot_Hash__Util__FieldHash (pTHX_ CV* cv);
EXTERN_C void boot_HTML__Parser (pTHX_ CV* cv);
EXTERN_C void boot_IO (pTHX_ CV* cv);
EXTERN_C void boot_List__Util (pTHX_ CV* cv);
EXTERN_C void boot_MIME__Base64 (pTHX_ CV* cv);
EXTERN_C void boot_Opcode (pTHX_ CV* cv);
EXTERN_C void boot_PerlIO__scalar (pTHX_ CV* cv);
EXTERN_C void boot_PerlIO__via (pTHX_ CV* cv);
EXTERN_C void boot_Storable (pTHX_ CV* cv);
EXTERN_C void boot_Sys__Hostname (pTHX_ CV* cv);
EXTERN_C void boot_Tie__Hash__NamedCapture (pTHX_ CV* cv);
EXTERN_C void boot_Time__HiRes (pTHX_ CV* cv);
EXTERN_C void boot_arybase (pTHX_ CV* cv);
EXTERN_C void boot_attributes (pTHX_ CV* cv);
EXTERN_C void boot_mro (pTHX_ CV* cv);
EXTERN_C void boot_re (pTHX_ CV* cv);
#undef STATIC2
#endif
#ifdef STATIC3
    newXS("Cwd::bootstrap", boot_Cwd, file);
    newXS("Data::Dumper::bootstrap", boot_Data__Dumper, file);
    newXS("Devel::Peek::bootstrap", boot_Devel__Peek, file);
    newXS("Digest::MD5::bootstrap", boot_Digest__MD5, file);
    newXS("Digest::SHA::bootstrap", boot_Digest__SHA, file);
    newXS("File::Glob::bootstrap", boot_File__Glob, file);
    newXS("IO::bootstrap", boot_IO, file);
    newXS("Hash::Util::bootstrap", boot_Hash__Util, file);
    newXS("Hash::Util::FieldHash::bootstrap", boot_Hash__Util__FieldHash, file);
    newXS("HTML::Parser::bootstrap", boot_HTML__Parser, file);
    newXS("List::Util::bootstrap", boot_List__Util, file);
    newXS("MIME::Base64::bootstrap", boot_MIME__Base64, file);
    newXS("Opcode::bootstrap", boot_Opcode, file);
    newXS("PerlIO::scalar::bootstrap", boot_PerlIO__scalar, file);
    newXS("PerlIO::via::bootstrap", boot_PerlIO__via, file);
    newXS("Storable::bootstrap", boot_Storable, file);
    newXS("Sys::Hostname::bootstrap", boot_Sys__Hostname, file);
    newXS("Tie::Hash::NamedCapture::bootstrap", boot_Tie__Hash__NamedCapture, file);
    newXS("Time::HiRes::bootstrap", boot_Time__HiRes, file);
    newXS("arybase::bootstrap", boot_arybase, file);
    newXS("attributes::bootstrap", boot_attributes, file);
    newXS("mro::bootstrap", boot_mro, file);
    newXS("re::bootstrap", boot_re, file);
#undef STATIC3
#endif
