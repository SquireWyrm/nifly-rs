// Part of particles_shim.h: replacement classes for blocks misread in some versions.
#pragma once

#include "particles_shim.h"

namespace nifly_rs {
// Replacements for blocks that are misread in some versions. Each reads and
// writes like its base from the version that stores all its fields, and skips
// the fields older files don't store. The factories load the blocks as these,
// the add shims create them, and clones stay replacements.

// NiPSysEmitter reads and writes radiusVariation in every version, but files
// store it only from 10.4.0.1. Base is a concrete emitter type.
template<class Base>
class Emitter final: public Base {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_4_0_1) {
            Base::Get(stream);
            return;
        }
        nifly::NiPSysModifier::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync_old(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_4_0_1) {
            Base::Put(stream);
            return;
        }
        nifly::NiPSysModifier::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync_old(s);
    }

  private:
    // The emitter fields without radiusVariation, then the subclass fields.
    void sync_old(nifly::NiStreamReversible& s) {
        s.Sync(this->speed);
        s.Sync(this->speedVariation);
        s.Sync(this->declination);
        s.Sync(this->declinationVariation);
        s.Sync(this->planarAngle);
        s.Sync(this->planarAngleVariation);
        s.Sync(this->color);
        s.Sync(this->radius);
        s.Sync(this->lifeSpan);
        s.Sync(this->lifeSpanVariation);
        if constexpr (std::is_base_of_v<nifly::NiPSysVolumeEmitter, Base>)
            this->nifly::NiPSysVolumeEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysSphereEmitter>)
            this->nifly::NiPSysSphereEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysCylinderEmitter>)
            this->nifly::NiPSysCylinderEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysBoxEmitter>)
            this->nifly::NiPSysBoxEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysMeshEmitter>)
            this->nifly::NiPSysMeshEmitter::Sync(s);
    }

    Emitter* Clone_impl() const override {
        return new Emitter(*this);
    }
};

// NiPSysRotationModifier reads and writes the speed variation, angle, angle
// variation and random sign in every version, but files store them only from
// 20.0.0.2.
class RotationModifier final: public nifly::NiPSysRotationModifier {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V20_0_0_2) {
            nifly::NiPSysRotationModifier::Get(stream);
            return;
        }
        nifly::NiPSysModifier::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync_old(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V20_0_0_2) {
            nifly::NiPSysRotationModifier::Put(stream);
            return;
        }
        nifly::NiPSysModifier::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync_old(s);
    }

  private:
    void sync_old(nifly::NiStreamReversible& s) {
        s.Sync(initialSpeed);
        s.Sync(randomInitialAxis);
        s.Sync(initialAxis);
    }

    RotationModifier* Clone_impl() const override {
        return new RotationModifier(*this);
    }
};

// NiMeshPSysData reads and writes its pool fields in every version, but files
// store them only from 10.2.0.0.
class MeshPSysData final: public nifly::NiMeshPSysData {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0) {
            nifly::NiMeshPSysData::Get(stream);
            return;
        }
        nifly::NiPSysData::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        nodeRef.Sync(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0) {
            nifly::NiMeshPSysData::Put(stream);
            return;
        }
        nifly::NiPSysData::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        nodeRef.Sync(s);
    }

  private:
    MeshPSysData* Clone_impl() const override {
        return new MeshPSysData(*this);
    }
};

// Loads every block of a type as the replacement T.
template<class T>
class ReplacementFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<T>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto b = std::make_unique<T>();
        b->Get(stream);
        return b;
    }
};
} // namespace nifly_rs
