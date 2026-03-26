<img width="1918" height="1079" alt="image" src="https://github.com/user-attachments/assets/8222263a-2c1b-4c15-99ad-aaf01d8cd8f6" />
# Remittance Visualizer

## VẤN ĐỀ
Người gửi tiền quốc tế (freelancer, du học sinh, lao động) phải chịu phí cao (~3–5%) và thời gian xử lý chậm (1–5 ngày) khi sử dụng hệ thống truyền thống như SWIFT.

---

## GIẢI PHÁP
dApp sử dụng Stellar để mô phỏng và thực hiện chuyển tiền với phí cực thấp và thời gian gần như tức thì, đồng thời hiển thị so sánh trực quan giữa phí SWIFT và Stellar.

---

## TÍNH NĂNG STELLAR SỬ DỤNG
- [x] Chuyển XLM/USDC  
- [ ] Token tùy chỉnh  
- [x] Soroban contract  
- [ ] DEX tích hợp  
- [ ] Trustline  
- [ ] Clawback/Tuân thủ  

---

## NGƯỜI DÙNG MỤC TIÊU
- Freelancer nhận tiền từ nước ngoài  
- Du học sinh  
- Người lao động gửi tiền về gia đình  

---

## TÍNH NĂNG CỐT LÕI (MVP)
Một giao dịch duy nhất:
- Gửi một khoản tiền (giả lập hoặc testnet)
- Hiển thị phí giao dịch trên Stellar
- So sánh trực tiếp với phí SWIFT (~5%)

---

## TẠI SAO STELLAR
- Phí giao dịch:
  - SWIFT: ~3–5%
  - Stellar: ~0.000003 XLM/giao dịch  

- Thời gian xử lý:
  - SWIFT: 1–5 ngày  
  - Stellar: vài giây  

→ Stellar giảm đáng kể chi phí và thời gian cho chuyển tiền quốc tế.

---

## CÔNG NGHỆ SỬ DỤNG
- Soroban Smart Contract (Rust)
- Stellar Testnet
- Stellar CLI

---

## CÁCH CHẠY PROJECT

### 1. Build contract
```bash
stellar contract build
