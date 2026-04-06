# บทนำ

## PRECC คืออะไร?

PRECC (การแก้ไขข้อผิดพลาดเชิงคาดการณ์สำหรับ Claude Code) เป็นเครื่องมือ Rust ที่ดักจับคำสั่ง bash ของ Claude Code ผ่านกลไก hook PreToolUse อย่างเป็นทางการ แก้ไขข้อผิดพลาด*ก่อนที่จะเกิดขึ้น* ประหยัดโทเค็นและกำจัดลูปการลองใหม่

ฟรีสำหรับผู้ใช้ชุมชน

## ปัญหา

Claude Code สูญเสียโทเค็นจำนวนมากกับข้อผิดพลาดที่ป้องกันได้:

- **ข้อผิดพลาดไดเรกทอรี** -- รัน `cargo build` ในไดเรกทอรีที่ไม่มี `Cargo.toml`
- **ลูปการลองใหม่** -- คำสั่งที่ล้มเหลวสร้างเอาต์พุตยาว
- **เอาต์พุตยาว** -- คำสั่งเช่น `find` หรือ `ls -R` สร้างหลายพันบรรทัด

## สี่เสาหลัก

### แก้ไขบริบท (cd-prepend)

ตรวจจับเมื่อคำสั่งเช่น `cargo build` หรือ `npm test` ทำงานในไดเรกทอรีผิดและเพิ่ม `cd /path/ที่ถูก &&` ก่อนการทำงาน

### การดีบัก GDB

ตรวจจับโอกาสในการแนบ GDB เพื่อดีบัก segfault และ crash

### การขุดเซสชัน

วิเคราะห์ล็อกเซสชัน Claude Code เพื่อหาคู่ความล้มเหลว-การแก้ไข

### ทักษะอัตโนมัติ

ไลบรารีของทักษะที่จับคู่รูปแบบคำสั่งและเขียนใหม่

## วิธีการทำงาน (เวอร์ชัน 30 วินาที)

1. Claude Code กำลังจะรันคำสั่ง bash
2. Hook PreToolUse ส่งคำสั่งเป็น JSON ไปยัง `precc-hook`
3. `precc-hook` ประมวลผลคำสั่งในเวลาน้อยกว่า 3 มิลลิวินาที
4. คำสั่งที่แก้ไขแล้วถูกส่งกลับเป็น JSON
5. Claude Code รันคำสั่งที่แก้ไขแล้ว

Claude ไม่เคยเห็นข้อผิดพลาด

### การบีบอัดแบบปรับตัว

หากคำสั่งล้มเหลวหลังจากการบีบอัด PRECC จะข้ามการบีบอัดในการลองใหม่โดยอัตโนมัติ เพื่อให้ Claude ได้เอาต์พุตเต็มสำหรับการดีบัก

## สถิติการใช้งานสด

เวอร์ชันปัจจุบัน <span data-stat="current_version">--</span>:

| เมตริก | ค่า |
|---|---|
| การเรียก Hook | <span data-stat="total_invocations">--</span> |
| โทเค็นที่ประหยัดได้ | <span data-stat="total_tokens_saved">--</span> |
| อัตราการประหยัด | <span data-stat="saving_pct">--</span>% |
| การเขียนใหม่ RTK | <span data-stat="rtk_rewrites">--</span> |
| การแก้ไข CD | <span data-stat="cd_prepends">--</span> |
| เวลาแฝง Hook | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| ผู้ใช้ | <span data-stat="unique_users">--</span> |

### Measured Savings (Ground Truth)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>เมตริก</th><th>ค่า</th></tr></thead>
<tbody>
<tr><td>Original output tokens (without PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Actual output tokens (with PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>โทเค็นที่ประหยัดได้</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>อัตราการประหยัด</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Ground-truth measurements</td><td><span data-measured="ground_truth_count">--</span> measurements</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### By Rewrite Type

<table id="rewrite-type-table">
<thead><tr><th>Type</th><th>Count</th><th>Avg Savings %</th><th>โทเค็นที่ประหยัดได้</th></tr></thead>
<tbody><tr><td colspan="4"><em>กำลังโหลด...</em></td></tr></tbody>
</table>
</div>

### การประหยัดตามรุ่น

<table id="version-breakdown" style="display:none">
<thead><tr><th>เวอร์ชัน</th><th>ผู้ใช้</th><th>การเรียก Hook</th><th>โทเค็นที่ประหยัดได้</th><th>อัตราการประหยัด</th></tr></thead>
<tbody><tr><td colspan="5"><em>กำลังโหลด...</em></td></tr></tbody>
</table>

<small>ตัวเลขเหล่านี้อัปเดตอัตโนมัติจากการวัดระยะไกลที่ไม่ระบุตัวตน</small>

## ลิงก์

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- เว็บไซต์: [https://peria.ai](https://peria.ai)
- เอกสาร: [https://precc.cc](https://precc.cc)
