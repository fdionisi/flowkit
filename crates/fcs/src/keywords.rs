pub enum RequiredKeyword {
    BeginAnalysis,
    BeginData,
    BeginsText,
    Byteord,
    DataType,
    EndAnalysis,
    EndData,
    EndsText,
    Mode,
    NextData,
    Par,
    Tot,
}

pub enum OptionalKeyword {
    Abrt,
    Btim,
    Cells,
    Com,
    Csmode,
    Csvbits,
    Cyt,
    Cytsn,
    Date,
    Etim,
    Exp,
    Fil,
    Gate,
    Inst,
    LastModified,
    LastModifier,
    Lost,
    Op,
    Originality,
    PlateId,
    PlateName,
    Proj,
    Smno,
    Spillover,
    Src,
    Sys,
    Timestep,
    Tr,
    Vol,
    WellId,
}

impl TryFrom<String> for RequiredKeyword {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "$BEGINANALYSIS" => Ok(RequiredKeyword::BeginAnalysis),
            "$BEGINDATA" => Ok(RequiredKeyword::BeginData),
            "$BEGINSTEXT" => Ok(RequiredKeyword::BeginsText),
            "$BYTEORD" => Ok(RequiredKeyword::Byteord),
            "$DATATYPE" => Ok(RequiredKeyword::DataType),
            "$ENDANALYSIS" => Ok(RequiredKeyword::EndAnalysis),
            "$ENDATA" => Ok(RequiredKeyword::EndData),
            "$ENDSTEXT" => Ok(RequiredKeyword::EndsText),
            "$MODE" => Ok(RequiredKeyword::Mode),
            "$NEXTDATA" => Ok(RequiredKeyword::NextData),
            "$PAR" => Ok(RequiredKeyword::Par),
            "$TOT" => Ok(RequiredKeyword::Tot),
            _ => Err("oh no!"),
        }
    }
}

impl ToString for RequiredKeyword {
    fn to_string(&self) -> String {
        match self {
            RequiredKeyword::BeginAnalysis => "$BEGINANALYSIS".into(),
            RequiredKeyword::BeginData => "$BEGINDATA".into(),
            RequiredKeyword::BeginsText => "$BEGINSTEXT".into(),
            RequiredKeyword::Byteord => "$BYTEORD".into(),
            RequiredKeyword::DataType => "$DATATYPE".into(),
            RequiredKeyword::EndAnalysis => "$ENDANALYSIS".into(),
            RequiredKeyword::EndData => "$ENDATA".into(),
            RequiredKeyword::EndsText => "$ENDSTEXT".into(),
            RequiredKeyword::Mode => "$MODE".into(),
            RequiredKeyword::NextData => "$NEXTDATA".into(),
            RequiredKeyword::Par => "$PAR".into(),
            RequiredKeyword::Tot => "$TOT".into(),
        }
    }
}

impl TryFrom<String> for OptionalKeyword {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "$ABRT" => Ok(OptionalKeyword::Abrt),
            "$BTIM" => Ok(OptionalKeyword::Btim),
            "$CELLS" => Ok(OptionalKeyword::Cells),
            "$COM" => Ok(OptionalKeyword::Com),
            "$CSMODE" => Ok(OptionalKeyword::Csmode),
            "$CSVBITS" => Ok(OptionalKeyword::Csvbits),
            "$CYT" => Ok(OptionalKeyword::Cyt),
            "$CYTSN" => Ok(OptionalKeyword::Cytsn),
            "$DATE" => Ok(OptionalKeyword::Date),
            "$ETIM" => Ok(OptionalKeyword::Etim),
            "$EXP" => Ok(OptionalKeyword::Exp),
            "$FIL" => Ok(OptionalKeyword::Fil),
            "$GATE" => Ok(OptionalKeyword::Gate),
            "$ISNT" => Ok(OptionalKeyword::Inst),
            "$LAST_MODIFIED" => Ok(OptionalKeyword::LastModified),
            "$LAST_MODIFIER" => Ok(OptionalKeyword::LastModifier),
            "$LOST" => Ok(OptionalKeyword::Lost),
            "$OP" => Ok(OptionalKeyword::Op),
            "$ORIGINALITY" => Ok(OptionalKeyword::Originality),
            "$PLATEID" => Ok(OptionalKeyword::PlateId),
            "$PLATENAME" => Ok(OptionalKeyword::PlateName),
            "$PROJ" => Ok(OptionalKeyword::Proj),
            "$SMNO" => Ok(OptionalKeyword::Smno),
            "$SPILLOVER" => Ok(OptionalKeyword::Spillover),
            "$SRC" => Ok(OptionalKeyword::Src),
            "$SYS" => Ok(OptionalKeyword::Sys),
            "$TIMESTEP" => Ok(OptionalKeyword::Timestep),
            "$TR" => Ok(OptionalKeyword::Tr),
            "$VOL" => Ok(OptionalKeyword::Vol),
            "$WELLID" => Ok(OptionalKeyword::WellId),
            _ => Err("oh no!"),
        }
    }
}

impl ToString for OptionalKeyword {
    fn to_string(&self) -> String {
        match self {
            OptionalKeyword::Abrt => "$ABRT".into(),
            OptionalKeyword::Btim => "$BTIM".into(),
            OptionalKeyword::Cells => "$CELLS".into(),
            OptionalKeyword::Com => "$COM".into(),
            OptionalKeyword::Csmode => "$CSMODE".into(),
            OptionalKeyword::Csvbits => "$CSVBITS".into(),
            OptionalKeyword::Cyt => "$CYT".into(),
            OptionalKeyword::Cytsn => "$CYTSN".into(),
            OptionalKeyword::Date => "$DATE".into(),
            OptionalKeyword::Etim => "$ETIM".into(),
            OptionalKeyword::Exp => "$EXP".into(),
            OptionalKeyword::Fil => "$FIL".into(),
            OptionalKeyword::Gate => "$GATE".into(),
            OptionalKeyword::Inst => "$ISNT".into(),
            OptionalKeyword::LastModified => "$LAST_MODIFIED".into(),
            OptionalKeyword::LastModifier => "$LAST_MODIFIER".into(),
            OptionalKeyword::Lost => "$LOST".into(),
            OptionalKeyword::Op => "$OP".into(),
            OptionalKeyword::Originality => "$ORIGINALITY".into(),
            OptionalKeyword::PlateId => "$PLATEID".into(),
            OptionalKeyword::PlateName => "$PLATENAME".into(),
            OptionalKeyword::Proj => "$PROJ".into(),
            OptionalKeyword::Smno => "$SMNO".into(),
            OptionalKeyword::Spillover => "$SPILLOVER".into(),
            OptionalKeyword::Src => "$SRC".into(),
            OptionalKeyword::Sys => "$SYS".into(),
            OptionalKeyword::Timestep => "$TIMESTEP".into(),
            OptionalKeyword::Tr => "$TR".into(),
            OptionalKeyword::Vol => "$VOL".into(),
            OptionalKeyword::WellId => "$WELLID".into(),
        }
    }
}
