// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: sammy <sam@bombus.cloud>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{unsafe_impl_qflag, QFlags};

#[cxx::bridge(namespace = "QSql")]
mod ffi {
    /// This enum type describes special SQL navigation locations.
    ///
    /// Qt docs for QSql::Location: https://doc.qt.io/qt-6/qsql.html#Location-enum
    #[repr(i32)]
    #[derive(Debug)]
    enum Location {
        /// Before the first record.
        BeforeFirstRow = -1,
        /// After the last record.
        AfterLastRow = -2,
    }

    /// This enum lists the policies for representing numerical values in the database with
    /// precisions greater than their corresponding C++/Rust types in the application.
    ///
    /// Note: The actual behaviour if an overflow occurs is driver specific. The Oracle
    /// database just returns an error in this case.
    ///
    /// Qt docs for QSql::NumericalPrecisionPolicy: https://doc.qt.io/qt-6/qsql.html#NumericalPrecisionPolicy-enum
    #[repr(u32)]
    #[derive(Debug)]
    enum NumericalPrecisionPolicy {
        /// Force 32bit integer values. In case of floating point numbers, the fractional
        /// part is silently discarded.
        LowPrecisionInt32 = 0b0001,
        /// Force 64bit integer values. In case of floating point numbers, the fractional
        /// part is silently discarded.
        LowPrecisionInt64 = 0b0010,
        /// Force double values. This is the default policy.
        LowPrecisionDouble = 0b0100,
        /// Strings will be used to preserve precision.
        HighPrecision = 0b0000,
    }

    /// This enum is used to specify the type of a bind parameter.
    ///
    /// Qt docs for Qt::ParamTypeFlag: https://doc.qt.io/qt-6/qsql.html#ParamTypeFlag-enum
    #[repr(u32)]
    #[derive(Debug)]
    enum ParamTypeFlag {
        /// The bind parameter is used to put data into the database.
        In = 0b0001,
        /// The bind parameter is used to receive data from the database.
        Out = 0b0010,
        /// The bind parameter is used to put data into the database; it will be overwritten
        /// with output data on executing a query.
        InOut = 0b0011,
        /// This must be OR'd with one of the other flags if you want to indicate that the
        /// data being transferred is raw binary data.
        Binary = 0b0100,
    }

    /// This enum type describes types of SQL tables.
    ///
    /// Qt docs for Qt::TableType: https://doc.qt.io/qt-6/qsql.html#TableType-enum
    #[repr(u32)]
    #[derive(Debug)]
    enum TableType {
        /// All the tables visible to the user.
        Tables = 0b0001,
        /// Internal tables used by the database.
        SystemTables = 0b0010,
        /// All the views visible to the user.
        Views = 0b0100,
        /// All of the above.
        AllTables = 0xff,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/sql/qsql.h");
        type Location;
        type NumericalPrecisionPolicy;
        type ParamTypeFlag;
        type TableType;
    }
}

pub use ffi::{
    Location, NumericalPrecisionPolicy, ParamTypeFlag, TableType,
};

/// [`QFlags`] of [`ParamTypeFlag`].
pub type ParamType = QFlags<ParamTypeFlag>;

unsafe_impl_qflag!(ParamTypeFlag, "QSql::ParamType", u32);
